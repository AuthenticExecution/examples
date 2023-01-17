use std::sync::Mutex;

lazy_static! {
    static ref HEATING_ON: Mutex<bool> = {
        Mutex::new(false) // initially OFF
    };

    static ref TEMPERATURE: Mutex<f32> = {
        Mutex::new(16.0) // initial value
    };
}

const MIN_TEMP : f32 = 14.0;
const MAX_TEMP : f32 = 30.0;

//@ sm_output(send_actual_temp)

//@ sm_entry
pub fn read_from_sensor(_data : &[u8]) -> ResultMessage {
    let heating_on = HEATING_ON.lock().unwrap();
    let mut temp = TEMPERATURE.lock().unwrap();

    match *heating_on {
        true if *temp < MAX_TEMP    => *temp += 0.1,
        false if *temp > MIN_TEMP   => *temp -= 0.1,
        _                           => ()
    }

    info!("Temperature: {}", *temp);
    send_actual_temp(&(*temp).to_be_bytes());
    success(None)
}

//@ sm_input
pub fn set_heating_state(data : &[u8]) {
    let mut heating_on = HEATING_ON.lock().unwrap();

    if data.len() < 2 {
        error!("get_heating_state: invalid data");
        return;
    }

    *heating_on = u16::from_be_bytes([data[0], data[1]]) != 0;
}