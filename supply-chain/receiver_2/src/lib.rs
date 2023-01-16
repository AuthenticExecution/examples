use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use mbedtls::pk::Pk;
use mbedtls::rng::Rdrand as Rng;
use sha2::{Sha256, Digest};
use json::object;

const RSA_BITS: u32 = 2048;
const EXPONENT: u32 = 0x10001;

lazy_static! {
    static ref RSA_KEY: Mutex<Pk> = {
        Mutex::new(Pk::generate_rsa(&mut Rng, RSA_BITS, EXPONENT).expect("Failed to generate key"))
    };

    static ref SHIPMENT_DATA: Mutex<Vec<u8>> = {
        Mutex::new(Vec::new())
    };
}

//@ sm_output(send_ack)

//@ sm_entry
pub fn init(_data : &[u8]) -> ResultMessage {
    info!("initializing SGX receiver");
    // trigger generation of keys
    let _ = RSA_KEY.lock().unwrap();
    success(None)
}

//@ sm_input
pub fn start_shipment(data : &[u8]) {
    debug!("Received: {:?}", data);
    let mut shipment_data = SHIPMENT_DATA.lock().unwrap();
    let mut rsa_key = RSA_KEY.lock().unwrap();

    if data.len() < 6 {
        error!("Wrong data received.");
        return;
    }

    // for the sake of the experiment, change key to have different outputs
    // at each iteration (so that no cached values would be used)
    *rsa_key = match Pk::generate_rsa(&mut Rng, RSA_BITS, EXPONENT) {
        Ok(e)   => e,
        Err(e)  => {
            error!("Failed to generate RSA key");
            panic!("{}", e);
        }
    };

    measure_time_ms("START_SHIPMENT");
    shipment_data.clear();

    // parse data into a JSON file
    let shipment = object!{
        sensor_id: u16::from_le_bytes([data[0], data[1]]),
        shipment_id: u16::from_le_bytes([data[2], data[3]]),
        data_id: u16::from_le_bytes([data[4], data[5]]),
        status: "start"
    };

    //info!("Shipment data: {:?}", json::stringify(shipment));

    shipment_data.extend_from_slice(json::stringify(shipment).as_bytes());
    send_ack(data);
}

//@ sm_input
pub fn start_shipment_complete(_data : &[u8]) {
    debug!("Received: {:?}", _data);
    let mut shipment_data = SHIPMENT_DATA.lock().unwrap();
    let mut rsa_key = RSA_KEY.lock().unwrap();

    debug!("Shipment data size: {}", shipment_data.len());
    measure_time_ms("END_TRANSMISSION");

    // SHA-256 of the shipment data
    let mut hasher = Sha256::new();
    hasher.update(&*shipment_data);
    let result = hasher.finalize();

    // signature of the hash using RSA
    let mut signature = [0u8; RSA_BITS as usize / 8];
    match rsa_key.sign(
        mbedtls::hash::Type::Sha256,
        &result,
        &mut signature,
        &mut Rng
    ) {
        Ok(_)   => (),
        Err(e)  => {
            error!(e);
            return;
        }
    }

    shipment_data.clear();
    measure_time_ms("START_SHIPMENT_COMPLETE");
}

// function for printing time to stdout
fn measure_time_ms(msg : &str) {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d)   => info!("{}: {} ms", msg, d.as_millis()),
        Err(_)  => info!("{}: ERROR", msg)
    }
}