#![no_main]
#![no_std]

extern crate alloc;

use ble::service::{CharacteristicInfo, Permission, Property};
use ch58x::ch58x as pac;
use ch58x_ble_rt as ble;
use ch58x_hal as hal;
use embedded_hal::digital::v2::OutputPin;

static NAME: &str = "blinky";
static SERVICE_UUID: &str = "4c430000-7ef2-424d-9cdb-a6527db702a0";
static BLINKY_WRITE_UUID: &str = "4c430001-7ef2-424d-9cdb-a6527db702a0";

pub struct BlinkyService {
    led: hal::PinB<4, hal::Output<hal::_5mA>>,
}
impl ble::service::Service for BlinkyService {
    fn write(
        &mut self,
        handle: u16,
        attribute: &ble::service::GattAttribute,
        data: &[u8],
        _offset: u16,
        method: u8,
    ) -> Result<(), ble::ble_const::AttributeError> {
        log::trace!(
            "W {} {:02x?} {method} {handle} {data:02x?}",
            attribute.uuid(),
            attribute.permission_mask(),
        );
        self.led.toggle();
        Ok(())
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
    led.set_high().unwrap();

    let service = ble::service::ServiceInfo::new(SERVICE_UUID.try_into().unwrap())
        .add_characteristic(
            CharacteristicInfo::new(BLINKY_WRITE_UUID.try_into().unwrap())
                .add_property(Property::GATT_PROP_WRITE_NO_RSP)
                .add_permission(Permission::GATT_PERMIT_WRITE),
        )
        .set_service(alloc::boxed::Box::new(BlinkyService { led }));

    ble::Config {
        scan_name: NAME,
        ..Default::default()
    }
    .init();
    ble::service::register(service);
    log::info!("Loop start");
    loop {
        ble::system_process();
    }
}
