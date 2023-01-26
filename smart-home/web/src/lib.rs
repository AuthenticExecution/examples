use std::thread;
use std::sync::Mutex;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use mbedtls::pk::Pk;
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context};
use mbedtls::x509::{Certificate};
use mbedtls::alloc::List as CertList;
use mbedtls::rng::Rdrand as Rng;
use std::sync::Arc;
use json::JsonValue;

use httparse::{EMPTY_HEADER, Request};

use authentic_execution::measure_time_us;

mod tls;
use tls::{CERTIFICATE};

mod error;
use error::ClientError;

mod status;
use status::Status;

mod webpage;

lazy_static! {
    static ref INIT: Mutex<bool> = {
        Mutex::new(false)
    };

    static ref TOKEN: Mutex<String> = {
        Mutex::new(String::new())
    };

    static ref STATUS: Mutex<Status> = {
        Mutex::new(Status::new())
    };
}

static HOST_NAME: &str = "node-sgx";

//@ sm_output(set_desired_temp)
//@ sm_output(enable_heating)
//@ sm_output(enable_switch)

//@ sm_input
pub fn set_status(data : &[u8]) {
    let mut status = STATUS.lock().unwrap();

    match serde_json::from_slice(data) {
        Ok(s)   => *status = s,
        Err(e)  => error!("Bad status received: {}", e)
    }

    measure_time_us("receive_status");
}

//@ sm_handler
pub fn init_server(data : &[u8]) -> Vec<u8> {
    let mut is_init = INIT.lock().unwrap();
    let mut token = TOKEN.lock().unwrap();

    if *is_init {
        error!("init input was already called");
        return vec!();
    }

    if data.len() <= 2 {
        error!("wrong data received (expecting: <port><token>)");
        return vec!();
    }

    let port = authentic_execution::data_to_u16(data);
    let host = format!("0.0.0.0:{}", port);

    *token = match std::str::from_utf8(&data[2..]) {
        Ok(t)   => t.to_string(),
        Err(e)  => {
            error!("Bad token: {}", e);
            return vec!();
        }
    };

    let listener = match TcpListener::bind(host) {
        Ok(l)   => l,
        Err(e)  => {
            error!("Fatal error: {}", e);
            return vec!();
        }
    };

    let (key, cert) = match tls::init_credentials() {
        Ok((k, c))      => (k,c),
        Err(e)          => {
            error!("Error with credentials: {}", e);
            return vec!();
        }
    };

    info!("Web server listening on 0.0.0.0:{}", port);
    *is_init = true;
    thread::spawn(move || { start_server(listener, key, cert) });

    let cert = CERTIFICATE.lock().unwrap();
    cert.as_ref().unwrap().clone()
}

fn start_server(listener : TcpListener, key : Pk, cert : CertList<Certificate>) {
    let rng = Arc::new(Rng);

    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    config.set_rng(rng);

    if let Err(_) = config.push_cert(Arc::new(cert), Arc::new(key)) {
        error!("Could not push cert in config");
        return;
    }

    let rc_config = Arc::new(config);

    for stream in listener.incoming() {
        if let Ok(s) = stream {
            if let Err(_e) = handle_client(s, rc_config.clone()) {
                debug!("Client error: {}", _e);
            }
        }
    }
}

fn handle_client(conn : TcpStream, config : Arc<Config>) -> anyhow::Result<()> {
    debug!("New client. Establishing TLS");
    let mut ctx = Context::new(config);
    ctx.establish(conn, None)?;

    debug!("Reading data");
    let mut buffer = [0; 4096];
    let bytes_read = ctx.read(&mut buffer)?;

    // parse HTTP request
    debug!("Parsing HTTP request");
    let mut headers = [EMPTY_HEADER; 1024];
    let mut req = Request::new(&mut headers);
    let req_status = req.parse(&buffer)?;

    if !req_status.is_complete() {
        return Err(ClientError::IncompleteHttpRequest.into());
    }

    // check if method exists
    let method = match req.method {
        Some(m) => m,
        None    => return Err(ClientError::MissingMethod.into())
    };

    //info!("Path: {:?} method: {}", req.path, method);
    //info!("Headers: {:?}", req.headers);

    let mut response : Vec<u8> = Vec::new();

    // implement API
    debug!("Serving request");
    measure_time_us("serving_http_request");
    
    match req.path {
        // main page
        Some(p) if p == "/" && method == "GET"                      => {
            response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
            response.extend_from_slice(webpage::MAIN.as_bytes());
        }
        // all subsequent API calls need to be authenticated
        _ if !check_token(&req)                                     => {
            response.extend_from_slice("HTTP/1.1 401 Unauthorized\r\n\r\n".as_bytes());
        }
        // get current status of heater and temperature sensor
        Some(p) if p == "/get-status" && method == "GET"      => {
            let status = STATUS.lock().unwrap();

            match serde_json::to_vec(&*status) {
                Ok(mut s)   => {
                    response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                    response.append(&mut s);
                },
                _       => response.extend_from_slice(
                    "HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes()
                )
            }
        }
        // set desired temperature, enabling automatic heating
        Some(p) if p == "/set-desired-temp" && method == "POST"      => {
            match parse_json_body(&buffer[req_status.unwrap()..bytes_read]) {
                Ok(b) if b["temp"].as_f32().is_some()   => {
                    let temp = b["temp"].as_f32().unwrap();

                    response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                    set_desired_temp(&temp.to_le_bytes());
                }
                _  => response.extend_from_slice(
                    "HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()
                )
            }
        }
        // enable/disable heating, disabling automatic heating 
        Some(p) if p == "/enable-heating" && method == "POST"      => {
            match parse_json_body(&buffer[req_status.unwrap()..bytes_read]) {
                Ok(b) if b["enable"].as_bool().is_some()   => {
                    let enable = b["enable"].as_bool().unwrap();

                    response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                    enable_heating(&(enable as u16).to_le_bytes())
                }
                _  => response.extend_from_slice(
                    "HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()
                )
            }
        }
        // enable/disable light switch 
        Some(p) if p == "/enable-switch" && method == "POST"      => {
            match parse_json_body(&buffer[req_status.unwrap()..bytes_read]) {
                Ok(b) if b["enable"].as_bool().is_some()   => {
                    let enable = b["enable"].as_bool().unwrap();

                    response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                    enable_switch(&(enable as u16).to_le_bytes())
                }
                _  => response.extend_from_slice(
                    "HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()
                )
            }
        }
        _                                                           => {
            response.extend_from_slice("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
        }
    };

    debug!("Sending response");
    ctx.write(&response)?;
    ctx.flush()?;

    Ok(())
}

fn check_token(req : &Request) -> bool {
        // authenticate request by checking Authorization header (bearer token)
        let token = TOKEN.lock().unwrap();

        let auth_header = match req.headers.iter().find(|&h| h.name == "Authorization") {
            Some(t) => t,
            None    => return false
        };
    
        if std::str::from_utf8(auth_header.value) != Ok(&format!("Bearer {}", token)) {
            return false;
        }

        return true;
}

fn parse_json_body(body : &[u8]) -> anyhow::Result<JsonValue> {
    let str_body = std::str::from_utf8(body)?;
    let json_object = json::parse(str_body)?;
    Ok(json_object)
}