#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ch58x_hal::println!("{info}");
    loop {}
}
