//@ sm_output(fw_set_desired_temp)

//@ sm_input
pub fn rc_set_desired_temp(data : &[u8]) {
    info!("Forwarding set_desired_temp");
    fw_set_desired_temp(data);
}

//@ sm_output(fw_get_desired_temp)

//@ sm_input
pub fn rc_get_desired_temp(data : &[u8]) {
    info!("Forwarding get_desired_temp");
    fw_get_desired_temp(data);
}

//@ sm_output(fw_get_current_temp)

//@ sm_input
pub fn rc_get_current_temp(data : &[u8]) {
    info!("Forwarding get_current_temp");
    fw_get_current_temp(data);
}