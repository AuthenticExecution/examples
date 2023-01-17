use std::sync::Mutex;

mod status;
use status::Status;

lazy_static! {
    static ref STATUS: Mutex<Status> = {
        Mutex::new(Status::new())
    };
}

//@ sm_output(set_heating)
//@ sm_output(set_switch)
//@ sm_output(send_status)

//@ sm_entry
pub fn check_heater(_data : &[u8]) -> ResultMessage {
    let status = STATUS.lock().unwrap();

    // if automatic heating is enabled, check if heater needs to be toggled
    if status.auto_heating {
        let desired_state = status.actual_temp < status.desired_temp;

        if desired_state != status.heating_on {
            debug!("Setting thermostat to: {}", desired_state);
            set_heating(&(desired_state as u16).to_be_bytes());
        }
    }

    send_status_info(&*status);
    success(None)
}

/* received from thermostat */
//@ sm_input
pub fn set_heating_state(data : &[u8]) {
    let mut status = STATUS.lock().unwrap();

    if data.len() < 2 {
        error!("set_heating_state: invalid data");
        return;
    }

    status.heating_on = u16::from_be_bytes([data[0], data[1]]) != 0;
    send_status_info(&*status);
}

/* received from temp sensor */
//@ sm_input
pub fn set_actual_temp(data : &[u8]) {
    let mut status = STATUS.lock().unwrap();

    if data.len() < 4 {
        error!("set_actual_temp: invalid data");
        return;
    }

    status.actual_temp = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    send_status_info(&*status);
}

/* received from light switch */
//@ sm_input
pub fn set_switch_state(data : &[u8]) {
    let mut status = STATUS.lock().unwrap();

    if data.len() < 2 {
        error!("set_switch_state: invalid data");
        return;
    }

    status.switch_on = u16::from_be_bytes([data[0], data[1]]) != 0;
    send_status_info(&*status);
}

/* received from web */
//@ sm_input
pub fn set_desired_temp(data : &[u8]) {
    let mut status = STATUS.lock().unwrap();

    if data.len() < 4 {
        error!("set_desired_temp: invalid data");
        return;
    }

    status.desired_temp = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    status.auto_heating = true;
    send_status_info(&*status);
}

//@ sm_input
pub fn enable_heating(data : &[u8]) {
    let mut status = STATUS.lock().unwrap();

    if data.len() < 2 {
        error!("enable_heating: invalid data");
        return;
    }

    status.auto_heating = false;
    set_heating(data);
    send_status_info(&*status);
}

//@ sm_input
pub fn enable_switch(data : &[u8]) {
    if data.len() < 2 {
        error!("enable_switch: invalid data");
        return;
    }

    set_switch(data);
}

fn send_status_info(status : &Status) {
    match serde_json::to_vec(status) {
        Ok(v)   => send_status(&v),
        Err(e)  => error!("Failed to serialize status: {}", e)
    }
}