use std::sync::Mutex;

lazy_static! {
    static ref NUM_REQUESTS: Mutex<u32> = {
        Mutex::new(0) // initially zero
    };
}

//@sm_input
pub fn increment_requests(_data : &[u8]) {
    let mut req = NUM_REQUESTS.lock().unwrap();
    *req += 1;
}

//@ sm_handler
pub fn get_num_requests(_data : &[u8]) -> Vec<u8> {
    info!("Retrieving number of requests");
    let req = NUM_REQUESTS.lock().unwrap();
    (*req).to_be_bytes().to_vec()
}
