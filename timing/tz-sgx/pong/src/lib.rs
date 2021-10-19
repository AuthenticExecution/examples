//@ sm_output(send_pong)

//@ sm_input
pub fn recv_ping(data : &[u8]) {
    info!(&format!("received ping with data size: {}", data.len()));

    // send data back
    send_pong(data);
}

//@ sm_handler
pub fn recv_ping_req(data : &[u8]) -> Vec<u8> {
    info!(&format!("received ping with data size: {}", data.len()));

    // send data back, as response
    data.to_vec()
}
