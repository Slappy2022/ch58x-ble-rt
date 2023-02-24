#![no_main]
#![no_std]

extern crate alloc;

use ble::service::{CharacteristicInfo, Permission, Property};
use ch58x::ch58x as pac;
use ch58x_ble_rt as ble;
use ch58x_hal as hal;
use embedded_hal::digital::v2::OutputPin;

static NAME: &str = "epoch";
static SERVICE_UUID: &str = "943e0000-23a2-4616-9db2-c7d0b1316c34";
static EPOCH_READ_UUID: &str = "943e0001-23a2-4616-9db2-c7d0b1316c34";

pub struct EpochService;
impl ble::service::Service for EpochService {
    fn read<'a>(
        &mut self,
        handle: u16,
        attribute: &ble::service::GattAttribute,
        buf: &'a mut [u8],
        _offset: u16,
        method: u8,
    ) -> Result<&'a [u8], ble::ble_const::AttributeError> {
        log::trace!(
            "R {} {:02x?} {method} {handle}",
            attribute.uuid(),
            attribute.permission_mask(),
        );
        let now = hal::now_us().to_le_bytes();
        if buf.len() < now.len() {
            return Err(ble::ble_const::AttributeError::ATT_ERR_INSUFFICIENT_RESOURCES);
        }
        let buf = &mut buf[..now.len()];
        buf.copy_from_slice(&now);
        Ok(buf)
    }
}

#[export_name = "rust_main"]
pub fn main() -> ! {
    hal::clock::set_sys_clock(hal::clock::ClockSource::Pll60MHz);
    let peripherals = unsafe { pac::Peripherals::steal() };
    let serial = {
        let uart = peripherals.UART3;
        let tx = hal::PinA::<5>::new().into_output_5ma();
        let rx = hal::PinA::<4>::new().into_output_5ma();
        hal::Serial::new(uart, tx, rx)
    };
    hal::println::init(serial);
    hal::logger::init(log::LevelFilter::Trace);
    log::info!("Logging initialized for {NAME}");

    let mut led = hal::PinB::<4>::new().into_output_5ma();

    let service = ble::service::ServiceInfo::new(SERVICE_UUID.try_into().unwrap())
        .add_characteristic(
            CharacteristicInfo::new(EPOCH_READ_UUID.try_into().unwrap())
                .add_property(Property::GATT_PROP_READ)
                .add_permission(Permission::GATT_PERMIT_READ),
        )
        .set_service(alloc::boxed::Box::new(EpochService));

    ble::Config {
        enable_advert: true,
        scan_name: NAME,
        scan_min_connection_interval: 8,
        scan_max_connection_interval: 20,
        scan_power: 0,
        advert_data: true,
        min_connection_interval: 6,
        max_connection_interval: 1000,
        bond_manager: true,
        add_all_services: true,
        disable_broadcast: true,
        ..Default::default()
    }
    .init();
    ble::service::register(service);
    log::info!("Loop start");
    led.set_high().unwrap();
    loop {
        ble::system_process();
    }
}
