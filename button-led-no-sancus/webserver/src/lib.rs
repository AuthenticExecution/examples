use std::thread;
use std::sync::Mutex;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use mbedtls::pk::Pk;
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context};
use mbedtls::x509::{Certificate, Time};
use mbedtls::x509::certificate::Builder;
use mbedtls::alloc::List as CertList;
use mbedtls::hash::Type as MdType;
use mbedtls::rng::Rdrand as Rng;
use std::sync::Arc;

lazy_static! {
    static ref INIT: Mutex<bool> = {
        Mutex::new(false)
    };

    static ref PRIVATE_KEY: Mutex<Pk> = {
        Mutex::new(Pk::generate_rsa(&mut Rng, 2048, 65537).expect("Failed to generate key"))
    };

    static ref CERTIFICATE: Mutex<Option<Vec<u8>>> = {
        Mutex::new(None)
    };
}

static HOST_NAME: &str = "node-sgx";

//@ sm_request(get_presses)

//@ sm_handler
pub fn init_server(data : &[u8]) -> Vec<u8> {
    let mut is_init = INIT.lock().unwrap();

    if *is_init {
        error!("init input was already called");
        return vec!();
    }

    if data.len() != 2 {
        error!("wrong data received (expecting 2 bytes for port)");
        return vec!();
    }

    let port = authentic_execution::data_to_u16(data);
    let host = format!("0.0.0.0:{}", port);

    let listener = match TcpListener::bind(host) {
        Ok(l)   => l,
        Err(e)  => {
            error!(&format!("Fatal error: {}", e));
            return vec!();
        }
    };

    let (key, cert) = match init_credentials() {
        Ok((k, c))      => (k,c),
        Err(e)          => {
            error!(&format!("Error with credentials: {}", e));
            return vec!();
        }
    };

    info!(&format!("Web server listening on 0.0.0.0:{}", port));
    thread::spawn(move || { start_server(listener, key, cert) });
    *is_init = true;

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
                warning!(&format!("Client error: {}", e));
            }
        }
    }
}

fn init_credentials() -> anyhow::Result<(Pk, CertList<Certificate>)> {
    let mut buf_key = [0u8; 8192];
    let mut key = PRIVATE_KEY.lock().unwrap();
    key.write_private_pem(&mut buf_key)?.unwrap();

    let key  = Pk::from_private_key(&buf_key, None)?;
    let key1 = Pk::from_private_key(&buf_key, None)?;
    let key2 = Pk::from_private_key(&buf_key, None)?;

    let cert_bytes = generate_cert(key1, key2)?;
    let cert = Certificate::from_pem_multiple(&cert_bytes)?;

    let mut certificate = CERTIFICATE.lock().unwrap();
    certificate.replace(cert_bytes);

    Ok((key, cert))
}

fn generate_cert(mut key1 : Pk, mut key2 : Pk) -> anyhow::Result<Vec<u8>> {
    let mut builder = Builder::new();
    let mut buf_cert = [0u8; 8192];

    let common_name = format!("CN={}\0", HOST_NAME);

    let cert = builder
        .subject_key(&mut key1)
        .subject_with_nul(&common_name)?
        .issuer_key(&mut key2)
        .issuer_with_nul(&common_name)?
        .validity(
            Time::new(2020, 1, 1, 0, 0, 0).unwrap(),
            Time::new(2030, 12, 31, 23, 59, 59).unwrap(),
        )?
        .serial(&[5])?
        .signature_hash(MdType::Sha256)
        .write_pem(&mut buf_cert, &mut Rng)?.unwrap();

    Ok(cert.to_vec())
}

fn handle_client(conn : TcpStream, config : Arc<Config>) -> anyhow::Result<()> {
    let mut ctx = Context::new(config);
    ctx.establish(conn, None)?;

    let mut buffer = [0; 1024];
    ctx.read(&mut buffer)?;

    let response = match get_presses(&[]) {
        Ok(r) if r.len() == 4   => {
            let num_presses = authentic_execution::data_to_u32(&r);
            format!("HTTP/1.1 200 OK\r\n\r\n{}\n", num_presses)
        },
        _                       => {
            "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string()
        }
    };

    ctx.write(response.as_bytes())?;
    ctx.flush()?;

    Ok(())
}
