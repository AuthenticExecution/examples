//@ sm_output(send_pong)

//@ sm_input
pub fn recv_ping(data : &[u8]) {
    info!("received ping");

    // send data back
    send_pong(data);
}

//@ sm_handler
pub fn recv_ping_req(data : &[u8]) -> Vec<u8> {
    info!("received ping");

    // send data back, as response
    data.to_vec()
}
