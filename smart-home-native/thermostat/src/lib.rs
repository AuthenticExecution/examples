use std::sync::Mutex;

lazy_static! {
    static ref DESIRED_TEMPERATURE: Mutex<f32> = {
        Mutex::new(18.0) // means off
    };

    static ref ACTUAL_TEMPERATURE: Mutex<f32> = {
        Mutex::new(100.0) // initial value
    };

    static ref HEATING_ON: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };
}

//@ sm_output(send_heating_state)

//@ sm_entry
pub fn check_heater(_data : &[u8]) -> ResultMessage {
    let mut heating_on = HEATING_ON.lock().unwrap();
    let desired_temp = DESIRED_TEMPERATURE.lock().unwrap();
    let actual_temp = ACTUAL_TEMPERATURE.lock().unwrap();
    

    if *actual_temp >= *desired_temp {
        *heating_on = false;
    } else {
        *heating_on = true;
    }

    info!(&format!("Heating on: {}", *heating_on));
    send_heating_state(&(*heating_on as u16).to_be_bytes());
    success(None)
}

//@ sm_input
pub fn get_actual_temp(data : &[u8]) {
    let mut actual_temp = ACTUAL_TEMPERATURE.lock().unwrap();

    if data.len() < 4 {
        error!("get_actual_temp: invalid data");
        return;
    }

    *actual_temp = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
}

//@ sm_input
pub fn get_desired_temp(data : &[u8]) {
    let mut desired_temp = DESIRED_TEMPERATURE.lock().unwrap();

    if data.len() < 4 {
        error!("get_desired_temp: invalid data");
        return;
    }

    *desired_temp = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
}