use std::sync::Mutex;
use mbedtls::pk::Pk;
use mbedtls::rng::Rdrand as Rng;
use sha2::{Sha256, Digest};
use reactive_crypto::{encrypt, Encryption};

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

    static ref SENSOR_INFO: Mutex<Vec<u8>> = {
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
    authentic_execution::measure_time_ms("start_shipment");
}

//@ sm_input
pub fn end_shipment(_data : &[u8]) {
    authentic_execution::measure_time_ms("end_shipment");
}

//@ sm_input
pub fn receive_sensor_data(data : &[u8]) {
    let mut sensor_data = SENSOR_DATA.lock().unwrap();
    let len = data.len();

    if len == 2 {
        let index = u16::from_le_bytes([data[0], data[1]]);

        if index > 0 {
            debug!(&format!("Start receiving sensor data. Num parts: {}", index));
            authentic_execution::measure_time_ms("START_SENSOR_DATA");
            sensor_data.clear();
        } else {
            debug!("Finished receiving sensor data.");
            let aes_key = AES_KEY.lock().unwrap();
            let mut rsa_key = RSA_KEY.lock().unwrap();
            let sensor_info = SENSOR_INFO.lock().unwrap();

            debug!(&format!("Data len: {}", sensor_data.len()));

            //authentic_execution::measure_time_ms("pre-encrypt");
            
            // encrypt all with AES
            let cipher = match encrypt(
                &sensor_data, 
                &aes_key, 
                &sensor_info, 
                &Encryption::Aes
            ) {
                Ok(c)   => c,
                Err(e)  => {
                    error!(e);
                    return;
                }
            };

            //authentic_execution::measure_time_ms("pre-hash");

            // SHA-256 of the encrypted data
            let mut hasher = Sha256::new();
            hasher.update(&cipher);
            let result = hasher.finalize();

            //authentic_execution::measure_time_ms("pre-sign");

            // signature of the hash
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

            // symmetric key encryption
            //TODO use a different key (does it matter?)
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
            authentic_execution::measure_time_ms("END_SENSOR_DATA");
        }
    } else {
        // actual sensor data
        debug!(&format!("Received sensor data part with size: {}", len));
        sensor_data.extend_from_slice(data);
    }
}