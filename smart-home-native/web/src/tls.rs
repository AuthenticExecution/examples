use sgx_attestation::mbedtls;
use mbedtls::pk::Pk;
use mbedtls::x509::{Certificate, Time};
use mbedtls::x509::certificate::Builder;
use mbedtls::alloc::List as CertList;
use mbedtls::hash::Type as MdType;
use mbedtls::rng::Rdrand as Rng;
use std::sync::Mutex;

use crate::HOST_NAME;

lazy_static! {
    static ref PRIVATE_KEY: Mutex<Pk> = {
        Mutex::new(Pk::generate_rsa(&mut Rng, 2048, 65537).expect("Failed to generate key"))
    };

    pub static ref CERTIFICATE: Mutex<Option<Vec<u8>>> = {
        Mutex::new(None)
    };
}

pub fn init_credentials() -> anyhow::Result<(Pk, CertList<Certificate>)> {
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
