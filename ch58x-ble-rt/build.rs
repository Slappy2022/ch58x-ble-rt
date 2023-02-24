fn main() {
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=src_c");
    println!("cargo:rerun-if-changed=startup.S");
    println!("cargo:rustc-link-lib=static=CH58xBLE");
    println!("cargo:rustc-link-lib=static=ISP583");
    println!("cargo:rustc-link-search=native=src_c");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .ctypes_prefix("core::ffi")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .header("src_c/ble.h")
        .header("src_c/service.h")
        .allowlist_function("BleInit")
        .allowlist_function("GAPBondMgr_PasscodeRsp")
        .allowlist_function("GAPRole_PeripheralInit")
        .allowlist_function("GAPRole_SetParameter")
        .allowlist_function("GATTServApp_AddService")
        .allowlist_function("GGS_AddService")
        .allowlist_function("TMOS_SystemProcess")
        .allowlist_function("TMOS_TimerInit")
        .allowlist_function("disable_broadcast")
        .allowlist_function("register_service")
        .allowlist_function("get_mac_address")
        .allowlist_function("start_device")
        .allowlist_function("tmos_msg_deallocate")
        .allowlist_function("tmos_msg_receive")
        .allowlist_type("gattAttribute_t")
        .allowlist_type("service_ptr_t")
        .allowlist_type("service_info_t")
        .clang_arg("--target=riscv32-unknown-none-elf")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    cc::Build::new()
        .include("src_c")
        .files(["src_c/ble.c", "src_c/log.c", "src_c/service.c"])
        .flag("-Wno-unused-parameter")
        .compile("led");
    cc::Build::new()
        .file("startup_CH583.S")
        .flag("-Wno-unused-parameter")
        .flag("-march=rv32imac_zicsr_zifencei")
        .compile("startup");
    std::fs::write(out_path.join("link.x"), include_bytes!("link.x")).unwrap();
}
