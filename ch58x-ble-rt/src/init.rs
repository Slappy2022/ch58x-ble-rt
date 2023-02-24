use ch58x::ch58x as pac;
fn set_parameter(param: u16, buf: &[u8]) {
    unsafe { crate::bindings::GAPRole_SetParameter(param, buf.len() as _, buf.as_ptr() as *mut _) };
}
fn set_parameter_u8(param: u16, value: u8) {
    set_parameter(param, &[value]);
}
pub(crate) fn set_parameter_bool(param: u16, value: bool) {
    match value {
        true => set_parameter_u8(param, 0x01),
        false => set_parameter_u8(param, 0x00),
    }
}
fn set_parameter_u16(param: u16, value: u16) {
    set_parameter(param, &value.to_le_bytes());
}
fn set_parameter_u32(param: u16, value: u32) {
    set_parameter(param, &value.to_le_bytes());
}

pub struct Bytes<const N: usize> {
    data: [u8; N],
    len: usize,
}
impl<const N: usize> Bytes<N> {
    pub fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }
    pub fn put_byte(&mut self, byte: u8) {
        self.data[self.len] = byte;
        self.len += 1;
    }
    pub fn put_bytes(&mut self, bytes: &[u8]) {
        self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
    }
    pub fn put_u16_le(&mut self, data: u16) {
        self.put_bytes(&data.to_le_bytes());
    }
    pub fn data(&self) -> &[u8] {
        &self.data[..self.len]
    }
    pub fn _raw_data(&self) -> [u8; N] {
        self.data
    }
}

pub struct Config {
    pub mac: Option<[u8; 6]>,
    pub enable_advert: bool,
    pub scan_name: &'static str,
    pub scan_min_connection_interval: u16,
    pub scan_max_connection_interval: u16,
    pub scan_power: u8,
    pub min_connection_interval: u16,
    pub max_connection_interval: u16,
    pub advert_data: bool,
    pub device_name_att: &'static str,
    pub bond_manager: bool,
    pub add_all_services: bool,
    pub disable_broadcast: bool,
    pub tx_power: crate::ble_const::TxPower,
}

impl core::default::Default for Config {
    fn default() -> Self {
        Self {
            mac: None,
            enable_advert: true,
            scan_name: "",
            scan_min_connection_interval: 8,
            scan_max_connection_interval: 20,
            scan_power: 0,
            min_connection_interval: 6,
            max_connection_interval: 1000,
            advert_data: true,
            device_name_att: "",
            bond_manager: false,
            add_all_services: false,
            disable_broadcast: false,
            tx_power: crate::ble_const::TxPower::LL_TX_POWEER_0_DBM,
        }
    }
}

impl Config {
    pub fn set_scan_rsp_data(&self) {
        use crate::ble_const::*;
        let mut buf = super::Bytes::<31>::new();
        let name = self.scan_name.as_bytes();
        buf.put_byte(name.len() as u8 + 1); //length
        buf.put_byte(GAP_ADTYPE_LOCAL_NAME_COMPLETE);
        buf.put_bytes(name);
        buf.put_byte(5); //length
        buf.put_byte(GAP_ADTYPE_SLAVE_CONN_INTERVAL_RANGE);
        buf.put_u16_le(self.scan_min_connection_interval);
        buf.put_u16_le(self.scan_max_connection_interval);
        buf.put_byte(2); //length
        buf.put_byte(GAP_ADTYPE_POWER_LEVEL);
        buf.put_byte(self.scan_power);
        set_parameter(GAPROLE_SCAN_RSP_DATA, buf.data());
    }
    pub fn set_advert_data(&self) {
        use crate::ble_const::*;
        if !self.advert_data {
            return;
        }
        let mut buf = super::Bytes::<31>::new();
        buf.put_byte(2); //length
        buf.put_byte(GAP_ADTYPE_FLAGS);
        buf.put_byte(GAP_ADTYPE_FLAGS_GENERAL | GAP_ADTYPE_FLAGS_BREDR_NOT_SUPPORTED);
        buf.put_byte(3); //length
        buf.put_byte(GAP_ADTYPE_16BIT_MORE);
        buf.put_u16_le(0x1800);
        set_parameter(GAPROLE_ADVERT_DATA, buf.data());
    }
    pub fn set_bond_manager(&self) {
        use crate::ble_const::*;
        if !self.bond_manager {
            return;
        }
        let passkey = 0u32;
        let pair_mode = GAPBOND_PAIRING_MODE_WAIT_FOR_REQ;
        let mitm = true;
        let bonding = true;
        let io_cap = GAPBOND_IO_CAP_DISPLAY_ONLY;
        set_parameter_u32(GAPBOND_PERI_DEFAULT_PASSCODE, passkey);
        set_parameter_u8(GAPBOND_PERI_PAIRING_MODE, pair_mode);
        set_parameter_bool(GAPBOND_PERI_MITM_PROTECTION, mitm);
        set_parameter_bool(GAPBOND_PERI_BONDING_ENABLED, bonding);
        set_parameter_u8(GAPBOND_PERI_IO_CAPABILITIES, io_cap);
    }
    pub fn add_all_services(&self) {
        if !self.add_all_services {
            return;
        }
        unsafe {
            use crate::ble_const::*;
            crate::bindings::GGS_AddService(GATT_ALL_SERVICES);
            crate::bindings::GATTServApp_AddService(GATT_ALL_SERVICES);
        }
    }
    pub fn disable_broadcast(&self) {
        if !self.disable_broadcast {
            return;
        }
        unsafe { crate::bindings::disable_broadcast() };
    }
    pub fn init(self) {
        use crate::ble_const::*;
        let mac = match self.mac {
            Some(mac) => mac,
            None => default_mac(),
        };
        log::trace!(
            "Starting ble with mac {:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}",
            mac[5],
            mac[4],
            mac[3],
            mac[2],
            mac[1],
            mac[0],
        );
        unsafe {
            crate::bindings::BleInit(mac.as_ptr() as *mut _, self.tx_power as u8)
        };
        sys_tick_config();
        ch58x_hal::safe::ck32k_config::clock_32k_source(ch58x_hal::safe::Clock32kSource::Internal);
        ch58x_hal::clock::calibrate(ch58x_hal::clock::CalibrationLevel::Level64);
        unsafe { crate::bindings::TMOS_TimerInit(core::ptr::null_mut()) };
        unsafe { crate::bindings::GAPRole_PeripheralInit() };
        unsafe { crate::bindings::start_device() };
        set_parameter_bool(GAPROLE_ADVERT_ENABLED, self.enable_advert);
        self.set_scan_rsp_data();
        self.set_advert_data();
        set_parameter_u16(GAPROLE_MIN_CONN_INTERVAL, self.min_connection_interval);
        set_parameter_u16(GAPROLE_MAX_CONN_INTERVAL, self.max_connection_interval);
        set_parameter(GGS_DEVICE_NAME_ATT, &self.device_name_att.as_bytes());
        self.set_bond_manager();
        self.add_all_services();
        self.disable_broadcast();
    }
}

pub fn sys_tick_config() {
    let load_mask_lo: u32 = 0xFFFF_FFFE;
    let load_mask_hi: u32 = 0xFFFF_FFFF;
    static CTLR_INIT: u32 = 1 << 5;
    static CTLR_STRE: u32 = 1 << 3;
    static CTLR_STCLK: u32 = 1 << 2;
    static CTLR_STIE: u32 = 1 << 1;
    static CTLR_STE: u32 = 1 << 0;
    unsafe {
        (*pac::SYSTICK::ptr()).cmplr.write(|w| w.bits(load_mask_lo));
        (*pac::SYSTICK::ptr()).cmphr.write(|w| w.bits(load_mask_hi));
        (*pac::SYSTICK::ptr())
            .ctlr
            .write(|w| w.bits(CTLR_INIT | CTLR_STRE | CTLR_STCLK | CTLR_STIE | CTLR_STE));
    }
}

fn default_mac() -> [u8; 6] {
    let mut mac = [0u8; 6];
    unsafe { crate::bindings::get_mac_address(mac.as_mut_ptr()) };
    mac
}

#[no_mangle]
pub fn sys_random() -> u32 {
    (ch58x_hal::sys::ticks() & 0xFFFF_FFFF) as u32
}
