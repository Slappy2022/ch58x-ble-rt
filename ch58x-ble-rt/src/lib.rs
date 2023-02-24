#![no_std]
#![feature(slice_ptr_get)]

mod bindings;
pub mod ble_const;
mod bond;
mod event;
mod init;
mod panic;
mod peripheral;
mod rt;
pub mod service;

pub use init::*;

pub fn system_process() {
    unsafe { bindings::TMOS_SystemProcess() };
}

#[no_mangle]
pub unsafe extern "C" fn rust_log(data: *const u8, len: usize) {
    let string = core::str::from_utf8_unchecked(core::slice::from_raw_parts(data, len));
    ch58x_hal::println!("{string}");
}
