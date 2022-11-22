//@ sm_input
pub fn start_shipment(_data : &[u8]) {
    info!("start_shipment");
    authentic_execution::measure_time("start_shipment");
}

//@ sm_input
pub fn end_shipment(_data : &[u8]) {
    info!("end_shipment");
    authentic_execution::measure_time("end_shipment");
}

//@ sm_input
pub fn receive_sensor_data(_data : &[u8]) {
    info!("receive_sensor_data");
    authentic_execution::measure_time("receive_sensor_data");
    //TODO
}