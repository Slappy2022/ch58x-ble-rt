use bluer::gatt::remote::Characteristic;
use std::time::Duration;

static NAME: &str = "epoch";
static SERVICE_UUID: &str = "943e0000-23a2-4616-9db2-c7d0b1316c34";
static EPOCH_READ_UUID: &str = "943e0001-23a2-4616-9db2-c7d0b1316c34";

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
    let characteristic =
        get_characteristic_by_path(&adapter, NAME, SERVICE_UUID, EPOCH_READ_UUID).await?;
    loop {
        let remote_start = read_duration(&characteristic).await?;
        let local_start = std::time::SystemTime::now();
        for count in 1.. {
            //std::thread::sleep(Duration::from_millis(1000));
            let remote = read_duration(&characteristic)
                .await?
                .saturating_sub(remote_start);
            let local = local_start.elapsed().unwrap();
            let diff = remote.saturating_sub(local);
            let neg_diff = local.saturating_sub(remote);
            let total_diff = diff.div_f64(count.into());
            let total_neg_diff = neg_diff.div_f64(count.into());
            match diff.is_zero() {
                true => println!("-{neg_diff:?}\t -{total_neg_diff:?}"),
                false => println!("{diff:?} -{total_diff:?}"),
            }
        }
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
