pub use rustls::Certificate;
pub use rustls::PrivateKey;

use super::{Result, ResultExt};

use std::io::{BufReader, Read};
use std::fs::File;
use std::path::Path;
use rustls::internal::pemfile;

pub fn read_cert_file_pem(path: &Path) -> Result<Certificate> {
    let file = File::open(path)
        .chain_err(|| format!("failed to open certificate file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut certs = match pemfile::certs(&mut r) {
        Ok(certs) => certs,
        Err(()) => bail!(format!("failed to read certificate {:?} for some reason", path)),
    };
    if certs.len() != 1 {
        bail!("expected 1 certificate in {:?}; got {}", path, certs.len());
    }
    Ok(certs.swap_remove(0))
}

pub fn read_cert_file_der(path: &Path) -> Result<Certificate> {
    let mut file = File::open(path)
        .chain_err(|| format!("failed to open certificate file {:?}", path))?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)
        .chain_err(|| format!("failed to read certificate file {:?}", path))?;
    Ok(Certificate(bytes))
}

pub fn read_key_file_pem(path: &Path) -> Result<PrivateKey> {
    let file = File::open(path)
        .chain_err(|| format!("failed to open private key file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut keys = match pemfile::pkcs8_private_keys(&mut r) {
        Ok(keys) => keys,
        Err(()) => bail!(format!("failed to read private key {:?} for some reason", path)),
    };
    if keys.len() != 1 {
        bail!("expected 1 private key in {:?}; got {}", path, keys.len());
    }
    Ok(keys.swap_remove(0))
}

pub fn read_key_file_der(path: &Path) -> Result<PrivateKey> {
    let mut file = File::open(path)
        .chain_err(|| format!("failed to open private key file {:?}", path))?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)
        .chain_err(|| format!("failed to read private key file {:?}", path))?;
    Ok(PrivateKey(bytes))
}
