use crate::bindings::gattAttribute_t;

pub struct SysMsg {
    msg: *mut u8,
}
impl SysMsg {
    pub fn receive(task_id: u8) -> Option<Self> {
        let msg: *mut u8 = core::ptr::null_mut();
        if unsafe { crate::bindings::tmos_msg_receive(task_id, msg) } != 0 {
            return None;
        }
        if msg.is_null() {
            return None;
        }
        Some(Self { msg })
    }
}
impl core::fmt::Display for SysMsg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut msg = self.msg;
        unsafe {
            while *msg != 0x00 {
                write!(f, "{}", *msg as char)?;
                msg = msg.offset(1);
            }
        }

        Ok(())
    }
}
impl core::ops::Drop for SysMsg {
    fn drop(&mut self) {
        unsafe { crate::bindings::tmos_msg_deallocate(self.msg) };
    }
}

static SYS_EVENT_MESSAGE: u16 = 0x8000;

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn rust_process_event(task_id: u8, mut events: u16) -> u16 {
    log::debug!("event:\t {task_id}\t{events}");
    if events & SYS_EVENT_MESSAGE != 0x00 {
        events ^= SYS_EVENT_MESSAGE;
        let msg = SysMsg::receive(task_id);
        if let Some(msg) = msg {
            log::info!("Sys event: {msg}");
        }
    }
    0
}

#[allow(non_snake_case, unused)]
#[no_mangle]
unsafe extern "C" fn ble_auth_callback_internal(
    connHandle: u16,
    pAttr: *mut gattAttribute_t,
    opcode: u8,
) -> u8 {
    log::trace!("ble_auth_callback_internal");
    0
}
