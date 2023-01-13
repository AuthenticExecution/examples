use std::sync::Mutex;

lazy_static! {
    static ref HEATING_ON: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };

    static ref TEMPERATURE: Mutex<f32> = {
        Mutex::new(16.0) // initial value
    };
}

//@ sm_output(send_actual_temp)

//@ sm_entry
pub fn read_from_sensor(_data : &[u8]) -> ResultMessage {
    let heating_on = HEATING_ON.lock().unwrap();
    let mut temp = TEMPERATURE.lock().unwrap();

    if *heating_on {
        *temp += 0.1;
    } else {
        *temp -= 0.1;
    }

    info!(&format!("Temperature: {}", *temp));
    send_actual_temp(&(*temp).to_be_bytes());
    success(None)
}

//@ sm_input
pub fn get_heating_state(data : &[u8]) {
    let mut heating_on = HEATING_ON.lock().unwrap();

    if data.len() < 1 {
        error!("get_heating_state: invalid data");
        return;
    }

    *heating_on = u16::from_be_bytes([data[0], data[1]]) != 0;
}