//@ sm_output(fw_int0)
//@ sm_output(fw_int1)

//@ sm_input
pub fn send_int0(data : &[u8]) {
    // Data must be forwarded to interface 0
    info!("Sending packet to interface 0");
    fw_int0(data);
}

//@ sm_input
pub fn send_int1(data : &[u8]) {
    // Data must be forwarded to interface 1
    info!("Sending packet to interface 1");
    fw_int1(data);
}
