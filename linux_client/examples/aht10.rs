use bluer::gatt::remote::Characteristic;
use std::time::Duration;

static NAME: &str = "aht10";
static SERVICE_UUID: &str = "181a";
static TEMP_READ_UUID: &str = "2a6e";
static HUMID_READ_UUID: &str = "2a6f";

pub async fn read_duration(c: &Characteristic) -> bluer::Result<Duration> {
    let bytes = c.read().await?;
    let us = u64::from_le_bytes(bytes.as_slice().try_into().expect("Couldn't parse u64"));
    let secs = us / 1_000_000;
    let us = (us % 1_000_000) as u32;
    Ok(Duration::new(secs, us))
}

async fn attempt(adapter: &bluer::Adapter) -> bluer::Result<()> {
    use linux_client::*;
    {
        let device = get_device_by_name(&adapter, NAME).await?;
        print_device_info(&device).await?;
        if !device.is_connected().await? {
            device.connect().await?;
        }
        print_services(&device).await?;
        let service = get_service_by_uuid(&device, SERVICE_UUID).await?;
        print_characteristics(&service).await?;
    }
    loop {
        let celsius = {
            let characteristic =
                get_characteristic_by_path(&adapter, NAME, SERVICE_UUID, TEMP_READ_UUID).await?;
            let bytes = characteristic.read().await?;
            let celsius_cents = i16::from_le_bytes(bytes.try_into().unwrap()) as f32;
            celsius_cents / 100.0
        };
        let rh = {
            let characteristic =
                get_characteristic_by_path(&adapter, NAME, SERVICE_UUID, HUMID_READ_UUID).await?;
            let bytes = characteristic.read().await?;
            let rh_cents = u16::from_le_bytes(bytes.try_into().unwrap()) as f32;
            rh_cents / 100.0
        };
        println!("{celsius:.2}°C\t{rh:.2}% rh");
        std::thread::sleep(Duration::from_millis(1000));
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    use linux_client::*;
    let adapter = get_adapter().await?;
    remove_all_devices(&adapter).await?;
    loop {
        if let Err(e) = attempt(&adapter).await {
            println!("{e:?}");
        }
    }
}
