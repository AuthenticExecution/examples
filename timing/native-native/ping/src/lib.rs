//@ sm_output(send_ping)
//@ sm_request(send_ping_req)

//@ sm_input
pub fn recv_pong(_data : &[u8]) {
    info!("received pong");
    authentic_execution::measure_time("end");
}

//@ sm_entry
pub fn start(data : &[u8]) -> ResultMessage {
    if data.len() != 2 {
        error!("input `start` needs 2 bytes as input (packet length)");
        return failure(ResultCode::IllegalPayload, None);
    }

    authentic_execution::measure_time("start");

    // number of bytes is stored in data
    let data_size = u16::from_le_bytes([data[0], data[1]]);

    info!(&format!("starting ping-pong with data size: {}", data_size));
    send_ping(&vec![0; data_size as usize]);

    success(None)
}
