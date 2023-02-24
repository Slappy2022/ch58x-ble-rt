use int_enum::IntEnum;

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_state_notify_callback_internal(state: u32, opcode: *mut u8) {
    let state = state as u8;
    use crate::ble_const::GapMessageEvent::GAP_LINK_TERMINATED_EVENT;
    use crate::ble_const::GapRoleStates::{GAPROLE_ADVERTISING, GAPROLE_WAITING};
    use crate::ble_const::GAPROLE_ADVERT_ENABLED;
    use crate::init::set_parameter_bool;
    if state == GAPROLE_ADVERTISING as u8 {
        if *opcode == GAP_LINK_TERMINATED_EVENT as u8 {
            set_parameter_bool(GAPROLE_ADVERT_ENABLED, true);
        }
    } else if state == GAPROLE_WAITING as u8 {
        if *opcode == GAP_LINK_TERMINATED_EVENT as u8 {
            set_parameter_bool(GAPROLE_ADVERT_ENABLED, true);
        }
    }
    let state = match crate::ble_const::GapMessageEvent::from_int(state) {
        Ok(s) => s,
        Err(e) => {
            log::info!("{e}");
            return;
        }
    };
    log::trace!("state notify: {state:?}");
}
#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_rssi_read_callback_internal(connHandle: u16, newRSSI: u8) {
    log::trace!("ble_rssi_read_callback_internal");
}
#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_param_update_callback_internal(
    connHandle: u16,
    connInterval: u16,
    connSlaveLatency: u16,
    connTimeout: u16,
) {
    log::trace!("ble_param_update_callback_internal");
}
