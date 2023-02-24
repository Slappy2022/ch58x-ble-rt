use std::time::Duration;

static NAME: &str = "blinky";
static SERVICE_UUID: &str = "4c430000-7ef2-424d-9cdb-a6527db702a0";
static BLINKY_WRITE_UUID: &str = "4c430001-7ef2-424d-9cdb-a6527db702a0";

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
    let characteristic =
        get_characteristic_by_path(&adapter, NAME, SERVICE_UUID, BLINKY_WRITE_UUID).await?;
    loop {
        characteristic.write(&[0x00]).await?;
        std::thread::sleep(Duration::from_millis(300));
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
