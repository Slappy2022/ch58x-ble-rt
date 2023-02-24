#![no_main]
#![no_std]

extern crate alloc;

use aht10::AHT10;
use ble::service::{CharacteristicInfo, Permission, Property};
use ch58x::ch58x as pac;
use ch58x_ble_rt as ble;
use ch58x_hal as hal;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use embedded_hal::digital::v2::OutputPin;

const NAME: &str = "aht10";
const SERVICE_UUID: &str = "181a";
const TEMP_READ_UUID: &str = "2a6e";
const HUMID_READ_UUID: &str = "2a6f";

pub struct Aht10Service<I2C, Delay, E>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    Delay: DelayMs<u16>,
    E: core::fmt::Debug,
{
    aht10: AHT10<I2C, Delay>,
}
impl<I2C, Delay, E> ble::service::Service for Aht10Service<I2C, Delay, E>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    Delay: DelayMs<u16>,
    E: core::fmt::Debug,
{
    fn read<'a>(
        &mut self,
        _handle: u16,
        attribute: &ble::service::GattAttribute,
        buf: &'a mut [u8],
        _offset: u16,
        _method: u8,
    ) -> Result<&'a [u8], ble::ble_const::AttributeError> {
        let uuid = attribute.uuid().to_heapless_string();
        /*
        log::trace!(
            "R {uuid} {:02x?} {_method} {_handle}",
            attribute.permission_mask(),
        );
        */
        //log::trace!("{:.2}°C\t{:.2}% rh", t.celsius(), h.rh());
        let (h, t) = self.aht10.read().unwrap();
        let bytes = match uuid.as_str() {
            TEMP_READ_UUID => {
                let celsius = (t.celsius() * 100.0) as i16;
                celsius.to_le_bytes()
            }
            HUMID_READ_UUID => {
                let rh = (h.rh() * 100.0) as u16;
                rh.to_le_bytes()
            }
            _ => return Err(ble::ble_const::AttributeError::ATT_ERR_ATTR_NOT_FOUND),
        };
        if buf.len() < bytes.len() {
            return Err(ble::ble_const::AttributeError::ATT_ERR_INSUFFICIENT_RESOURCES);
        }
        let buf = &mut buf[..bytes.len()];
        buf.copy_from_slice(&bytes);
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
    let i2c = {
        let i2c = peripherals.I2C;
        let scl = hal::PinB::<13>::new().into_pull_up_input();
        let sda = hal::PinB::<12>::new().into_pull_up_input();
        hal::i2c::I2c::new(i2c, scl, sda)
    };
    let mut aht10 = AHT10::new(i2c, hal::delay::Delay).unwrap();
    aht10.read().unwrap();
    log::trace!("aht10 init");

    let service = ble::service::ServiceInfo::new(SERVICE_UUID.try_into().unwrap())
        .add_characteristic(
            CharacteristicInfo::new(TEMP_READ_UUID.try_into().unwrap())
                .add_property(Property::GATT_PROP_READ)
                .add_permission(Permission::GATT_PERMIT_READ),
        )
        .add_characteristic(
            CharacteristicInfo::new(HUMID_READ_UUID.try_into().unwrap())
                .add_property(Property::GATT_PROP_READ)
                .add_permission(Permission::GATT_PERMIT_READ),
        )
        .set_service(alloc::boxed::Box::new(Aht10Service { aht10 }));

    ble::Config {
        scan_name: NAME,
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
