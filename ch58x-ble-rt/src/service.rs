extern crate alloc;

use crate::bindings::gattAttribute_t;
use crate::ble_const::AttributeError;
use alloc::boxed::Box;
use heapless::Vec;

#[derive(PartialEq)]
pub enum Uuid {
    Short([u8; 2]),
    Vendor([u8; 16]),
}
impl Uuid {
    pub fn to_heapless_string(&self) -> heapless::String<36> {
        use core::fmt::Write;
        let mut result = heapless::String::<36>::new();
        let data = match self {
            Self::Short(data) => data.as_slice(),
            Self::Vendor(data) => data.as_slice(),
        };
        for i in 0..data.len() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                write!(result, "-").unwrap();
            }
            write!(result, "{:02x?}", data[data.len() - 1 - i]).unwrap();
        }
        result
    }
}
impl core::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let data = match self {
            Self::Short(data) => data.as_slice(),
            Self::Vendor(data) => data.as_slice(),
        };
        write!(f, "0x")?;
        for b in data.iter().rev() {
            write!(f, "{b:02x?}")?;
        }
        Ok(())
    }
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let data = match self {
            Self::Short(data) => data.as_slice(),
            Self::Vendor(data) => data.as_slice(),
        };
        for i in 0..data.len() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                write!(f, "-")?;
            }
            write!(f, "{:02x?}", data[data.len() - 1 - i])?;
        }
        Ok(())
    }
}

// TODO: Better error
#[derive(Debug)]
pub struct InvalidLength;
impl TryFrom<&[u8]> for Uuid {
    type Error = InvalidLength;
    fn try_from(uuid: &[u8]) -> Result<Self, Self::Error> {
        match uuid.len() {
            2 => Ok(Self::Short(uuid.try_into().map_err(|_| InvalidLength)?)),
            16 => Ok(Self::Vendor(uuid.try_into().map_err(|_| InvalidLength)?)),
            _ => Err(InvalidLength),
        }
    }
}

fn hex_to_nibble(n: u8) -> u8 {
    match n {
        b'0'..=b'9' => n - b'0',
        b'a'..=b'f' => 10 + n - b'a',
        _ => panic!("bad hex digit: {n}"),
    }
}

impl TryFrom<&str> for Uuid {
    type Error = InvalidLength;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let v = s.as_bytes();
        match v.len() {
            4 => {
                let mut result = [0u8; 2];
                result[0] = (hex_to_nibble(v[2]) << 4) | hex_to_nibble(v[3]);
                result[1] = (hex_to_nibble(v[0]) << 4) | hex_to_nibble(v[1]);
                Ok(Self::Short(result))
            }
            36 => {
                let mut result = [0u8; 16];
                let mut i = 0;
                let mut j = 0;
                while i < 16 {
                    if v[j] == b'-' {
                        j += 1;
                    }
                    result[15 - i] = (hex_to_nibble(v[j]) << 4) | hex_to_nibble(v[j + 1]);
                    i += 1;
                    j += 2;
                }
                Ok(Self::Vendor(result))
            }
            _ => Err(InvalidLength),
        }
    }
}

#[repr(u8)]
pub enum GattAttributePermission {
    Read = 0x01,         // Attribute is Readable
    Write = 0x02,        // Attribute is Writable
    AuthenRead = 0x04,   // Read requires Authentication
    AuthenWrite = 0x08,  // Write requires Authentication
    AuthorRead = 0x10,   // Read requires Authorization
    AuthorWrite = 0x20,  // Write requires Authorization
    EncryptRead = 0x40,  // Read requires Encryption
    EncryptWrite = 0x80, // Write requires Encryption
}
#[derive(Debug)]
pub struct GattAttribute {
    uuid: Uuid,
    permission_mask: u8,
}

impl GattAttribute {
    pub fn has_permission(&self, permission: GattAttributePermission) -> bool {
        self.permission_mask & permission as u8 != 0x00
    }
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    pub fn permission_mask(&self) -> u8 {
        self.permission_mask
    }
}

impl TryFrom<*mut gattAttribute_t> for GattAttribute {
    type Error = InvalidLength;
    fn try_from(attr: *mut gattAttribute_t) -> Result<Self, Self::Error> {
        let attr = match attr.is_null() {
            true => return Err(InvalidLength),
            false => unsafe { *attr },
        };
        let uuid = unsafe { core::slice::from_raw_parts(attr.type_.uuid, attr.type_.len as usize) };
        Ok(Self {
            uuid: uuid.try_into()?,
            permission_mask: attr.permissions,
        })
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ble_write_callback_internal(
    connHandle: u16,
    pAttr: *mut gattAttribute_t,
    pValue: *mut u8,
    len: u16,
    offset: u16,
    method: u8,
) -> u8 {
    let attribute: GattAttribute = match pAttr.try_into() {
        Ok(a) => a,
        Err(_) => return 0,
    };
    let data = unsafe { core::slice::from_raw_parts(pValue, len as usize) };
    match call_service_write(connHandle, &attribute, data, offset, method) {
        Ok(()) => 0,
        Err(e) => e as u8,
    }
}

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_read_callback_internal(
    connHandle: u16,
    pAttr: *mut gattAttribute_t,
    pValue: *mut u8,
    pLen: *mut u16,
    offset: u16,
    maxLen: u16,
    method: u8,
) -> u8 {
    let attribute: GattAttribute = match pAttr.try_into() {
        Ok(a) => a,
        Err(_) => return 0,
    };
    let buf = unsafe { core::slice::from_raw_parts_mut(pValue, maxLen as usize) };
    match call_service_read(connHandle, &attribute, buf, offset, method) {
        Ok(data) => {
            *pLen = data.len() as u16;
            0
        }
        Err(e) => e as u8,
    }
}

#[allow(non_camel_case_types, unused)]
#[repr(u8)]
pub enum Permission {
    GATT_PERMIT_READ = 0x01,
    GATT_PERMIT_WRITE = 0x02,
    GATT_PERMIT_AUTHEN_READ = 0x04,
    GATT_PERMIT_AUTHEN_WRITE = 0x08,
    GATT_PERMIT_AUTHOR_READ = 0x10,
    GATT_PERMIT_AUTHOR_WRITE = 0x20,
    GATT_PERMIT_ENCRYPT_READ = 0x40,
    GATT_PERMIT_ENCRYPT_WRITE = 0x80,
}

#[allow(non_camel_case_types, unused)]
#[repr(u8)]
pub enum Property {
    GATT_PROP_BCAST = 0x01,
    GATT_PROP_READ = 0x02,
    GATT_PROP_WRITE_NO_RSP = 0x04,
    GATT_PROP_WRITE = 0x08,
    GATT_PROP_NOTIFY = 0x10,
    GATT_PROP_INDICATE = 0x20,
    GATT_PROP_AUTHEN = 0x40,
    GATT_PROP_EXTENDED = 0x80,
}

pub struct CharacteristicInfo {
    uuid: Uuid,
    property_mask: u8,
    permission_mask: u8,
}
impl CharacteristicInfo {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            property_mask: 0x00,
            permission_mask: 0x00,
        }
    }
    pub fn add_property(mut self, property: Property) -> Self {
        self.property_mask |= property as u8;
        self
    }
    pub fn add_permission(mut self, permission: Permission) -> Self {
        self.permission_mask |= permission as u8;
        self
    }
}
#[allow(unused)]
pub trait Service {
    fn write(
        &mut self,
        handle: u16,
        attribute: &GattAttribute,
        data: &[u8],
        offset: u16,
        method: u8,
    ) -> Result<(), AttributeError> {
        Err(AttributeError::ATT_ERR_ATTR_NOT_FOUND)
    }
    fn read<'a>(
        &mut self,
        handle: u16,
        attribute: &GattAttribute,
        buf: &'a mut [u8],
        offset: u16,
        method: u8,
    ) -> Result<&'a [u8], AttributeError> {
        Err(AttributeError::ATT_ERR_ATTR_NOT_FOUND)
    }
}
pub struct ServiceInfo {
    uuid: Uuid,
    characteristics: alloc::vec::Vec<CharacteristicInfo>,
    service: Option<Box<dyn Service>>,
}
impl ServiceInfo {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            characteristics: alloc::vec::Vec::new(),
            service: None,
        }
    }
    pub fn add_characteristic(mut self, characteristic: CharacteristicInfo) -> Self {
        self.characteristics.push(characteristic);
        self
    }
    pub fn set_service(mut self, service: Box<dyn Service>) -> Self {
        self.service = Some(service);
        self
    }
}

const MAX_NUM_SERVICES: usize = 8;
static mut SERVICE: Vec<Box<dyn Service>, MAX_NUM_SERVICES> = Vec::new();
fn call_service_write(
    handle: u16,
    attribute: &GattAttribute,
    data: &[u8],
    offset: u16,
    method: u8,
) -> Result<(), AttributeError> {
    unsafe {
        for ref mut service in &mut SERVICE {
            match service.write(handle, attribute, data, offset, method) {
                Err(AttributeError::ATT_ERR_ATTR_NOT_FOUND) => continue,
                x => return x,
            }
        }
    };
    log::info!("write gatt not found {}", attribute.uuid);
    Err(AttributeError::ATT_ERR_ATTR_NOT_FOUND)
}
fn call_service_read<'a>(
    handle: u16,
    attribute: &GattAttribute,
    buf: &'a mut [u8],
    offset: u16,
    method: u8,
) -> Result<&'a [u8], AttributeError> {
    unsafe {
        for ref mut service in &mut SERVICE {
            match service.read(handle, attribute, buf, offset, method) {
                Err(AttributeError::ATT_ERR_ATTR_NOT_FOUND) => continue,
                Err(err) => return Err(err),
                // TODO: Borrow checker gets it wrong here, remove transmute when the borrow
                // checker improves
                Ok(x) => return Ok(core::mem::transmute(x)),
            }
        }
    };
    log::info!("read gatt not found {}", attribute.uuid);
    Err(AttributeError::ATT_ERR_ATTR_NOT_FOUND)
}

pub fn register(service: ServiceInfo) {
    use crate::bindings::*;
    let num_chars = service.characteristics.len();
    let mut out_chars = alloc::vec::Vec::<characteristic_info_t>::with_capacity(num_chars);
    for i in 0..num_chars {
        let other = &service.characteristics[i];
        let (uuid, is_short) = match other.uuid {
            Uuid::Short(short) => {
                let mut uuid = [0u8; 16];
                uuid[..short.len()].copy_from_slice(&short);
                (uuid, true)
            }
            Uuid::Vendor(uuid) => (uuid, false),
        };
        out_chars.push(characteristic_info_t {
            uuid,
            is_short: is_short as u8,
            properties: other.property_mask,
            permissions: other.permission_mask,
        });
    }
    let (uuid, is_short) = match service.uuid {
        Uuid::Short(short) => {
            let mut uuid = [0u8; 16];
            uuid[..short.len()].copy_from_slice(&short);
            (uuid, true)
        }
        Uuid::Vendor(uuid) => (uuid, false),
    };
    let out_service = service_info_t {
        uuid,
        is_short: is_short as u8,
        characteristicsLen: num_chars,
        characteristics: out_chars.as_ptr(),
    };

    let ptr = service_ptr_t {
        ptr: core::ptr::null_mut(),
    };
    let out_service = &mut [out_service];
    let ptr = &mut [ptr];
    if let Some(service) = service.service {
        // Drop the service on the floor if adding fails
        let _ = unsafe { SERVICE.push(service) };
    }
    unsafe { crate::bindings::register_service(ptr.as_mut_ptr(), out_service.as_ptr()) };
}
