use crate::error::Error;

use std::{
    io::{Read, Write},
    result::Result,
};

use ring::{rand, signature::Ed25519KeyPair};
use zbox::{OpenOptions, RepoOpener};

pub fn generate_keypair(name: &str) -> Result<Ed25519KeyPair, Error> {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)?;

    match Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()) {
        Ok(keypair) => {
            store_key(pkcs8_bytes.as_ref(), name)?;
            Ok(keypair)
        }
        Err(_) => Err(Error::Unspecified),
    }
}

const STORAGE_URI: &str = "file://./storage";
const STORAGE_PASSWORD: &str = "your password"; // TODO: find place for secrets

fn get_private_key_path(name: &str) -> String {
    format!("/{}.private-key", name)
}

fn store_key(pkcs8_bytes: &[u8], name: &str) -> Result<(), Error> {
    // create and open a repository
    let mut repo = RepoOpener::new()
        .create(true)
        .open(STORAGE_URI, STORAGE_PASSWORD)?;

    // create and open a file for writing
    let mut file = OpenOptions::new()
        .create(true)
        .open(&mut repo, get_private_key_path(name))?;

    // use std::io::Write trait to write data into it
    file.write_all(pkcs8_bytes)?;

    // finish writing to make a permanent content version
    file.finish()?;

    Ok(())
}

pub fn retrieve_keypair_from_storage(name: &str) -> Result<Ed25519KeyPair, Error> {
    let pkcs8_bytes = retrieve_private_key_from_storage(name)?;

    match Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()) {
        Ok(keypair) => Ok(keypair),
        Err(_) => Err(Error::Unspecified),
    }
}

fn retrieve_private_key_from_storage(name: &str) -> Result<Vec<u8>, Error> {
    // create and open a repository
    let mut repo = RepoOpener::new()
        .create(true)
        .open(STORAGE_URI, STORAGE_PASSWORD)
        .unwrap();

    // create and open a file in repository for writing
    let mut file = OpenOptions::new()
        .create(false)
        .open(&mut repo, get_private_key_path(name))
        .unwrap();

    // read file content using std::io::Read trait
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    Ok(buffer)
}
