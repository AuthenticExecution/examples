//@ sm_output(send_ping)
//@ sm_request(send_ping_req)

//@ sm_input
pub fn recv_pong(_data : &[u8]) {
    info!("received pong");

    // do nothing (or we can restart the cycle again by sending a new ping)
}

//@ sm_entry
pub fn start(data : &[u8]) -> ResultMessage {
    // number of bytes is stored in data
    let data_size = u16::from_le_bytes([data[0], data[1]]);

    info!(&format!("starting ping-pong with data size: {}", data_size));
    send_ping(&vec![0; data_size as usize]);

    success(None)
}
