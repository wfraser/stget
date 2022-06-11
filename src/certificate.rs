pub use rustls::Certificate;
pub use rustls::PrivateKey;

use anyhow::{bail, Context, Result};

use std::io::{BufReader, Read};
use std::fs::File;
use std::path::Path;
use rustls_pemfile as pemfile;

pub fn read_cert_file_pem(path: &Path) -> Result<Certificate> {
    let file = File::open(path)
        .with_context(|| format!("failed to open certificate file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut certs = pemfile::certs(&mut r)
        .with_context(|| format!("failed to read certificate {path:?}"))?;
    if certs.len() != 1 {
        bail!("expected 1 certificate in {:?}; got {}", path, certs.len());
    }
    Ok(Certificate(certs.swap_remove(0)))
}

pub fn read_cert_file_der(path: &Path) -> Result<Certificate> {
    let mut file = File::open(path)
        .with_context(|| format!("failed to open certificate file {:?}", path))?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)
        .with_context(|| format!("failed to read certificate file {:?}", path))?;
    Ok(Certificate(bytes))
}

pub fn read_key_file_pem(path: &Path) -> Result<PrivateKey> {
    let file = File::open(path)
        .with_context(|| format!("failed to open private key file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut keys = pemfile::pkcs8_private_keys(&mut r)
        .with_context(|| format!("failed to read private key {path:?}"))?;
    if keys.len() != 1 {
        bail!("expected 1 private key in {:?}; got {}", path, keys.len());
    }
    Ok(PrivateKey(keys.swap_remove(0)))
}

pub fn read_key_file_der(path: &Path) -> Result<PrivateKey> {
    let mut file = File::open(path)
        .with_context(|| format!("failed to open private key file {:?}", path))?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)
        .with_context(|| format!("failed to read private key file {:?}", path))?;
    Ok(PrivateKey(bytes))
}
