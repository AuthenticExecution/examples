use std::sync::Mutex;

lazy_static! {
    static ref SWITCH_ON: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };
}

//@ sm_output(send_switch_state)

//@ sm_entry
pub fn check_switch(_data : &[u8]) -> ResultMessage {
    let switch_on = SWITCH_ON.lock().unwrap();

    debug!("Switch ON: {}", *switch_on);
    send_switch_state(&(*switch_on as u16).to_le_bytes());
    success(None)
}

//@ sm_input
pub fn set_switch(data : &[u8]) {
    let mut switch_on = SWITCH_ON.lock().unwrap();

    if data.len() < 2 {
        error!("set_switch_state: invalid data");
        return;
    }

    *switch_on = u16::from_le_bytes([data[0], data[1]]) != 0;
    send_switch_state(&(*switch_on as u16).to_le_bytes());
}