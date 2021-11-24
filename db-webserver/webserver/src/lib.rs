use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

//@ sm_output(request_received)
//@ sm_request(get_requests)

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

    let response = match get_requests(&[]) {
        Ok(r) if r.len() == 4   => {
            let num_presses = authentic_execution::data_to_u32(&r);
            format!("HTTP/1.1 200 OK\r\n\r\n{}\n", num_presses)
        },
        _                       => {
            "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string()
        }
    };

    if let Err(e) = conn.write(response.as_bytes()) {
        warning!(&format!("Client error: {}", e));
        return;
    }

    request_received(&[]);
}