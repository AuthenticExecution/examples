use std::sync::Mutex;
use json::object;

lazy_static! {
    static ref DESIRED_TEMPERATURE: Mutex<f32> = {
        Mutex::new(18.0) // initial value
    };

    static ref ACTUAL_TEMPERATURE: Mutex<f32> = {
        Mutex::new(100.0) // initial value
    };

    static ref AUTO_HEATING: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };

    static ref HEATING_ON: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };
}

//@ sm_output(set_heating)
//@ sm_output(send_status)

//@ sm_entry
pub fn check_heater(_data : &[u8]) -> ResultMessage {
    let auto_heating = AUTO_HEATING.lock().unwrap();
    let desired_temp = DESIRED_TEMPERATURE.lock().unwrap();
    let actual_temp = ACTUAL_TEMPERATURE.lock().unwrap();
    let heating_on = HEATING_ON.lock().unwrap();

    // send current status
    let status = object!{
        desired_temp: *desired_temp,
        actual_temp: *actual_temp,
        auto_heating: *auto_heating,
        heating_on: *heating_on
    };
    send_status(json::stringify(status).as_bytes());

    // if automatic heating is enabled, check if heater needs to be toggled
    if *auto_heating {
        let desired_state = *actual_temp < *desired_temp;

        if desired_state != *heating_on {
            debug!("Setting thermostat to: {}", desired_state);
            set_heating(&(desired_state as u16).to_be_bytes());
        }
    }

    success(None)
}

/* received from thermostat */
//@ sm_input
pub fn set_heating_state(data : &[u8]) {
    let mut heating_on = HEATING_ON.lock().unwrap();

    if data.len() < 2 {
        error!("get_heating_state: invalid data");
        return;
    }

    *heating_on = u16::from_be_bytes([data[0], data[1]]) != 0;
}

/* received from temp sensor */
//@ sm_input
pub fn set_actual_temp(data : &[u8]) {
    let mut actual_temp = ACTUAL_TEMPERATURE.lock().unwrap();

    if data.len() < 4 {
        error!("get_actual_temp: invalid data");
        return;
    }

    *actual_temp = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
}

/* received from web */
//@ sm_input
pub fn set_desired_temp(data : &[u8]) {
    let mut auto_heating = AUTO_HEATING.lock().unwrap();
    let mut desired_temp = DESIRED_TEMPERATURE.lock().unwrap();

    if data.len() < 4 {
        error!("get_desired_temp: invalid data");
        return;
    }

    *desired_temp = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    *auto_heating = true;
}

//@ sm_input
pub fn enable_heating(data : &[u8]) {
    let mut auto_heating = AUTO_HEATING.lock().unwrap();

    if data.len() < 2 {
        error!("enable_heating: invalid data");
        return;
    }

    *auto_heating = false;
    set_heating(data);
}