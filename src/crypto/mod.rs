use hex::encode;
use ring::{
    rand,
    signature::{self, Ed25519KeyPair, KeyPair},
};

use std::result::Result;

use crate::error::Error;

pub fn generate_keypair() -> Result<Ed25519KeyPair, Error> {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

    let pkcs8_string: String = encode(pkcs8_bytes.as_ref());

    let keypair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref());

    if keypair.is_err() {
        return Err(Error::Unspecified);
    }
    let keypair = keypair.unwrap();

    let pulic_key_bytes = keypair.public_key().as_ref();
    let pulic_key_string: String = encode(pulic_key_bytes.as_ref());

    println!(
        "public key\n{}\n\nprivate key\n{}",
        pulic_key_string, pkcs8_string
    );

    Ok(keypair)
}

