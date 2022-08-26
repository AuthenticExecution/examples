//@ sm_output(send)

//@ sm_entry
pub fn start(data : &[u8]) -> ResultMessage {
    if data.len() != 2 {
        error!("input `start` needs 2 bytes as input (packet length)");
        return failure(ResultCode::IllegalPayload, None);
    }

    authentic_execution::measure_time("start");

    // number of bytes is stored in data
    let data_size = u16::from_le_bytes([data[0], data[1]]);

    info!(&format!("starting data flow with data size: {}", data_size));
    send(&vec![0; data_size as usize]);

    success(None)
}
