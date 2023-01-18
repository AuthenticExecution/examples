use std::sync::Mutex;

lazy_static! {
    static ref HEATING_ON: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };
}

//@ sm_output(send_heating_state)

//@ sm_input
pub fn set_heating(data : &[u8]) {
    let mut heating_on = HEATING_ON.lock().unwrap();

    if data.len() < 2 {
        error!("set_heating: invalid data");
        return;
    }

    *heating_on = u16::from_le_bytes([data[0], data[1]]) != 0;
    send_heating_state(data);
}