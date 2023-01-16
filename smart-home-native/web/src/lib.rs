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

use httparse::{EMPTY_HEADER, Request};
use serde_json::{Value};

mod tls;
use tls::{CERTIFICATE};

mod error;
use error::ClientError;

lazy_static! {
    static ref INIT: Mutex<bool> = {
        Mutex::new(false)
    };

    static ref TOKEN: Mutex<String> = {
        Mutex::new(String::new())
    };
}

static HOST_NAME: &str = "node-sgx";

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
            if let Err(e) = handle_client(s, rc_config.clone()) {
                warning!("Client error: {}", e);
            }
        }
    }
}

fn handle_client(conn : TcpStream, config : Arc<Config>) -> anyhow::Result<()> {
    let mut ctx = Context::new(config);
    ctx.establish(conn, None)?;

    let mut buffer = [0; 1024];
    ctx.read(&mut buffer)?;

    // parse HTTP request
    let mut headers = [EMPTY_HEADER; 512];
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

    // authenticate request by checking Authorization header (bearer token)
    let token = TOKEN.lock().unwrap();

    let auth_header = match req.headers.iter().find(|&h| h.name == "Authorization") {
        Some(t) => t,
        None    => return Err(ClientError::MissingAuthenticationHeader.into())
    };

    if std::str::from_utf8(auth_header.value) != Ok(&format!("Bearer {}", token)) {
        return Err(ClientError::InvalidToken.into());
    }

    let mut response : Vec<u8> = Vec::new();

    // implement API
    match req.path {
        Some(p) if p == "/" && method == "GET"                      => {
            response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\nHome!\n".as_bytes())
        }
        Some(p) if p == "/get-current-temp" && method == "GET"      => {
            response.extend_from_slice("HTTP/1.1 200 OK\r\n\r\nget-current-temp!\n".as_bytes())
        }
        _                                                           => {
            response.extend_from_slice("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes())
        }
    };

    ctx.write(&response)?;
    ctx.flush()?;

    Ok(())
}
