use bluer::gatt::remote::{Characteristic, Service};
use bluer::{Adapter, AdapterEvent, Device, Result};
use futures::{pin_mut, StreamExt};

const BATTERY_SERVICE_UUID: &str = "180f";
const BATTERY_READ_UUID: &str = "2a19";

pub async fn print_device_info(device: &Device) -> Result<()> {
    println!("    Address:            {}", device.address());
    println!("    Address type:       {}", device.address_type().await?);
    println!("    Name:               {:?}", device.name().await?);
    println!("    Icon:               {:?}", device.icon().await?);
    println!("    Class:              {:?}", device.class().await?);
    println!(
        "    UUIDs:              {:?}",
        device.uuids().await?.unwrap_or_default()
    );
    println!("    Paired:             {:?}", device.is_paired().await?);
    println!("    Connected:          {:?}", device.is_connected().await?);
    println!("    Trusted:            {:?}", device.is_trusted().await?);
    println!("    Modalias:           {:?}", device.modalias().await?);
    println!("    RSSI:               {:?}", device.rssi().await?);
    println!("    TX power:           {:?}", device.tx_power().await?);
    println!(
        "    Manufacturer data:  {:?}",
        device.manufacturer_data().await?
    );
    println!("    Service data:       {:?}", device.service_data().await?);
    println!();
    Ok(())
}

pub async fn print_characteristic(c: &Characteristic) -> bluer::Result<()> {
    print!("\t{} \t", &c.uuid().await?);
    let flags = c.flags().await?;
    if flags.broadcast {
        print!("broadcast ");
    }
    if flags.read {
        print!("read ");
    }
    if flags.write_without_response {
        print!("write_without_response ");
    }
    if flags.write {
        print!("write ");
    }
    if flags.notify {
        print!("notify ");
    }
    if flags.indicate {
        print!("indicate ");
    }
    if flags.authenticated_signed_writes {
        print!("authenticated_signed_writes ");
    }
    if flags.extended_properties {
        print!("extended_properties ");
    }
    if flags.reliable_write {
        print!("reliable_write ");
    }
    if flags.writable_auxiliaries {
        print!("writable_auxiliaries ");
    }
    if flags.encrypt_read {
        print!("encrypt_read ");
    }
    if flags.encrypt_write {
        print!("encrypt_write ");
    }
    if flags.encrypt_authenticated_read {
        print!("encrypt_authenticated_read ");
    }
    if flags.encrypt_authenticated_write {
        print!("encrypt_authenticated_write ");
    }
    if flags.secure_read {
        print!("secure_read ");
    }
    if flags.secure_write {
        print!("secure_write ");
    }
    if flags.authorize {
        print!("authorize ");
    }
    println!();
    Ok(())
}

pub async fn print_services(device: &Device) -> bluer::Result<()> {
    println!("Services:");
    for service in device.services().await? {
        println!("\t{}", &service.uuid().await?);
    }
    println!();
    Ok(())
}

pub async fn print_characteristics(service: &Service) -> bluer::Result<()> {
    println!("Characters for service {}", service.uuid().await?);
    for c in service.characteristics().await? {
        if let Err(e) = print_characteristic(&c).await {
            println!("{e:?}");
        }
    }
    Ok(())
}

pub async fn get_characteristic_by_uuid(
    service: &Service,
    uuid: &str,
) -> bluer::Result<Characteristic> {
    for c in service.characteristics().await? {
        let uuid = match uuid.len() {
            4 => format!("0000{uuid}-0000-1000-8000-00805f9b34fb"),
            _ => uuid.to_string(),
        };
        if c.uuid().await?.to_string() == uuid {
            return Ok(c);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Characteristic not found").into())
}

pub async fn get_service_by_uuid(device: &Device, uuid: &str) -> bluer::Result<Service> {
    for service in device.services().await? {
        let uuid = match uuid.len() {
            4 => format!("0000{uuid}-0000-1000-8000-00805f9b34fb"),
            _ => uuid.to_string(),
        };
        if service.uuid().await?.to_string() == uuid {
            return Ok(service);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Service not found").into())
}

pub async fn get_device_by_name(adapter: &Adapter, name: &str) -> bluer::Result<Device> {
    let discover = adapter.discover_devices().await?;
    pin_mut!(discover);
    loop {
        if let Some(event) = discover.next().await {
            if let AdapterEvent::DeviceAdded(addr) = event {
                let device = adapter.device(addr)?;
                let device_name = match device.name().await? {
                    Some(name) => name,
                    None => continue,
                };
                if device_name == name {
                    return Ok(device);
                }
            }
        }
    }
}

pub async fn get_adapter() -> Result<Adapter> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

pub async fn get_characteristic_by_path(
    adapter: &Adapter,
    device_name: &str,
    service_uuid: &str,
    characteristic_uuid: &str,
) -> bluer::Result<Characteristic> {
    let device = get_device_by_name(&adapter, device_name).await?;
    if !device.is_connected().await? {
        device.connect().await?;
    }
    let service = get_service_by_uuid(&device, service_uuid).await?;
    get_characteristic_by_uuid(&service, characteristic_uuid).await
}

pub async fn remove_all_devices(adapter: &Adapter) -> bluer::Result<()> {
    for a in adapter.device_addresses().await? {
        adapter.remove_device(a).await?;
    }
    Ok(())
}
pub async fn get_battery(adapter: &Adapter, device_name: &str) -> bluer::Result<u8> {
    let characteristic = get_characteristic_by_path(
        adapter,
        device_name,
        BATTERY_SERVICE_UUID,
        BATTERY_READ_UUID,
    )
    .await?;
    let bytes = characteristic.read().await?;
    Ok(u8::from_le_bytes(bytes.try_into().unwrap()))
}
