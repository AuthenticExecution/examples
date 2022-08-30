use std::sync::Mutex;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

lazy_static! {
    static ref NUM_REQUESTS: Mutex<u32> = {
        Mutex::new(0) // initially zero
    };
}

//@ sm_entry
pub fn init(_data : &[u8]) -> ResultMessage {
    info!("starting web server");

    let listener = match TcpListener::bind("0.0.0.0:80") {
        Ok(l)   => l,
        Err(e)  => {
            error!(&format!("Fatal error: {}", e));
            return failure(ResultCode::InternalError, None);
        }
    };

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(s) = stream {
                handle_client(s)
            }
        }
    });

    success(None)
}

fn handle_client(mut conn : TcpStream) { 
    let mut buffer = [0; 1024];

    if let Err(e) = conn.read(&mut buffer) {
        warning!(&format!("Client error: {}", e));
        return;
    }

    let mut req = NUM_REQUESTS.lock().unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\n", *req);
    *req += 1;

    if let Err(e) = conn.write(response.as_bytes()) {
        warning!(&format!("Client error: {}", e));
    }
}

//@ sm_handler
pub fn get_requests(_data : &[u8]) -> Vec<u8> {
    let req = NUM_REQUESTS.lock().unwrap();
    (*req).to_be_bytes().to_vec()
}

//@ sm_output(__transfer)

//@sm_entry
pub fn __save(_data : &[u8]) -> ResultMessage {
    let req = NUM_REQUESTS.lock().unwrap();
    __transfer(&(*req).to_be_bytes());
    success(None)
}

//@sm_input
pub fn __restore(data : &[u8]) {
    let mut req = NUM_REQUESTS.lock().unwrap();
    *req = authentic_execution::data_to_u32(data);
}