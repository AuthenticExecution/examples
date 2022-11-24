//@ sm_input
pub fn start_shipment(_data : &[u8]) {
    authentic_execution::measure_time_ms("start_shipment");
}

//@ sm_input
pub fn end_shipment(_data : &[u8]) {
    authentic_execution::measure_time_ms("end_shipment");
}

//@ sm_input
pub fn receive_sensor_data(data : &[u8]) {
    let len = data.len();

    if len == 2 {
        let index = u16::from_le_bytes([data[0], data[1]]);

        if index > 0 {
            info!(&format!("Start receiving sensor data. Num parts: {}", index));
            authentic_execution::measure_time_ms("START_SENSOR_DATA");
        } else {
            info!("Finished receiving sensor data.");
            // TODO: re-encrypt, hash and sign data
            authentic_execution::measure_time_ms("END_SENSOR_DATA");
        }
    } else {
        // actual sensor data
        // TODO: store data
        info!(&format!("Received sensor data part with size: {}", len));
    }
}