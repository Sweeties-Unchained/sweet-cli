use crate::error::Error;

use std::{
    io::{Read, Write},
    path::Path,
    result::Result,
};

use ring::{rand, signature::Ed25519KeyPair};
use zbox::{OpenOptions, Repo, RepoOpener};

mod password_io;
use password_io::{create_new_password, read_password};

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

fn get_private_key_path(name: &str) -> String {
    format!("/{}.private-key", name)
}

fn open_storage_repo() -> Result<Repo, Error> {
    let repo_location = get_keychain_location()?;

    let password = match Repo::exists(&repo_location)? {
        true => read_password("Enter password of your key chain")?,
        false => {
            println!("No key chain found on your device, a new one will be created. Please keep the password in a safe place.");
            create_new_password()?
        }
    };

    // create and open a repository
    let repo = RepoOpener::new()
        .create(true)
        .open(&repo_location, &password);

    match repo {
        Ok(repo) => Ok(repo),
        Err(error) => {
            match error {
                zbox::Error::Decrypt => {
                    println!("Incorrect password.")
                }
                _ => {}
            };

            Err(Error::ZboxError(error))
        }
    }
}

fn store_key(pkcs8_bytes: &[u8], name: &str) -> Result<(), Error> {
    let mut repo = open_storage_repo()?;

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
    let mut repo = open_storage_repo()?;

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

fn get_keychain_location() -> Result<String, Error> {
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    let home_dir = home::home_dir().ok_or(Error::KeyChainLocationError)?;
    let absolute_path_target = home_dir.join(".sweet");

    let current_path = Path::new(&current_dir);
    let target_path = Path::new(&absolute_path_target);

    let relative_path =
        pathdiff::diff_paths(target_path, current_path).ok_or(Error::KeyChainLocationError)?;

    let relative_path_string = format!("file://{}", relative_path.display());

    Ok(relative_path_string)
}
