use int_enum::IntEnum;

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_passcode_callback_internal(
    deviceAddr: *mut u8,
    connectionHandle: u16,
    uiInputs: u8,
    uiOutputs: u8,
) {
    log::trace!("ble_passcode_callback_internal");
    unsafe {
        crate::bindings::GAPBondMgr_PasscodeRsp(connectionHandle, 0x00 /*SUCCESS*/, 314159)
    };
}

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_pair_state_callback_internal(
    connectionHandle: u16,
    state: u8,
    status: u8,
) {
    log::trace!("ble_pair_state_callback_internal");
    let state = match crate::ble_const::PairingState::from_int(state) {
        Ok(s) => s,
        Err(e) => {
            log::info!("{e}");
            return;
        }
    };
    log::trace!("{state:?}");
}

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_oob_callback_internal(
    deviceAddr: *mut u8,
    connectionHandle: u16,
    r_local: *mut u8,
    c_local: *mut u8,
) {
    log::trace!("ble_oob_callback_internal");
}
