use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use mbedtls::pk::Pk;
use mbedtls::rng::Rdrand as Rng;
use sha2::{Sha256, Digest};
use aes_gcm::Aes128Gcm as AesGcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray, Payload};

const RSA_BITS: u32 = 2048;
const EXPONENT: u32 = 0x10001;

lazy_static! {
    static ref RSA_KEY: Mutex<Pk> = {
        Mutex::new(Pk::generate_rsa(&mut Rng, RSA_BITS, EXPONENT).expect("Failed to generate key"))
    };

    static ref AES_KEY: Mutex<Vec<u8>> = {
        Mutex::new(vec![0u8; 16])
    };

    static ref SENSOR_DATA: Mutex<Vec<u8>> = {
        Mutex::new(Vec::new())
    };

    static ref SENSOR_METADATA: Mutex<Vec<u8>> = {
        Mutex::new(Vec::new())
    };
}

//@ sm_entry
pub fn init(_data : &[u8]) -> ResultMessage {
    info!("initializing SGX receiver");
    // trigger generation of keys
    let _ = RSA_KEY.lock().unwrap();
    let _ = AES_KEY.lock().unwrap();
    success(None)
}

//@ sm_input
pub fn start_shipment(_data : &[u8]) {
    measure_time_ms("START_SHIPMENT");
    debug!(&format!("Received: {:?}", data));
}

//@ sm_input
pub fn end_shipment(_data : &[u8]) {
    measure_time_ms("END_SHIPMENT");
    debug!(&format!("Received: {:?}", data));
}

//@ sm_input
pub fn start_sensing(data : &[u8]) {
    let mut sensor_data = SENSOR_DATA.lock().unwrap();
    let mut sensor_metadata = SENSOR_METADATA.lock().unwrap();
    let mut aes_key = AES_KEY.lock().unwrap();
    let mut rsa_key = RSA_KEY.lock().unwrap();

    // for the sake of the experiment, change keys to have different outputs
    // at each iteration (so that no cached values would be used)
    if let Err(e) = getrandom::getrandom(&mut aes_key[..16]) {
        error!("Failed to generate AES key");
        panic!("{}", e);
    }

    *rsa_key = match Pk::generate_rsa(&mut Rng, RSA_BITS, EXPONENT) {
        Ok(e)   => e,
        Err(e)  => {
            error!("Failed to generate RSA key");
            panic!("{}", e);
        }
    };

    measure_time_ms("START_SENSING");
    info!(&format!("Metadata: {:?}", data));
    sensor_data.clear();
    sensor_metadata.clear();
    sensor_metadata.extend_from_slice(data);
}

//@ sm_input
pub fn end_sensing(_data : &[u8]) {
    measure_time_ms("END_TRANSMISSION");
    debug!(&format!("Received: {:?}", data));
    let mut sensor_data = SENSOR_DATA.lock().unwrap();
    let mut sensor_metadata = SENSOR_METADATA.lock().unwrap();
    let aes_key = AES_KEY.lock().unwrap();
    let mut rsa_key = RSA_KEY.lock().unwrap();

    debug!(&format!("Data len: {}", sensor_data.len()));
    
    // encrypt all using AES-GCM-128
    let key_arr = GenericArray::clone_from_slice(&aes_key);
    let nonce_arr = GenericArray::from_slice(&[0u8; 12]);
    let aes = AesGcm::new(&key_arr);

    let cipher = match aes.encrypt(
        nonce_arr,
        Payload{msg : &sensor_data, aad : &sensor_metadata}
    ) {
        Ok(v) => v,
        Err(e) => {
            error!(e);
            return;
        }
    };

    // SHA-256 of the encrypted data
    let mut hasher = Sha256::new();
    hasher.update(&cipher);
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

    // symmetric key encryption using RSA
    let mut cipher = vec![0u8; RSA_BITS as usize / 8];
    match rsa_key.encrypt(
        &aes_key,
        &mut cipher,
        &mut Rng
    ) {
        Ok(_)   => (),
        Err(e)  => {
            error!(e);
            return;
        }
    }

    sensor_data.clear();
    sensor_metadata.clear();
    measure_time_ms("END_SENSING");
}

//@ sm_input
pub fn receive_sensor_data(data : &[u8]) {
    let mut sensor_data = SENSOR_DATA.lock().unwrap();
    debug!(&format!("Received sensor data part with size: {}", data.len()));
    sensor_data.extend_from_slice(data);
}

// function for printing time to stdout
fn measure_time_ms(msg : &str) {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d)   => info!(&format!("{}: {} ms", msg, d.as_millis())),
        Err(_)  => info!(&format!("{}: ERROR", msg))
    }
}