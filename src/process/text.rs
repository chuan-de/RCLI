use crate::cli::TextSignFormat;
use crate::get_reader;
use anyhow::Result;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::fs;
use std::io::Read;
use std::path::Path;
use rand::rngs::OsRng;

pub trait TestSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool>;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let sign = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(&sign);
    Ok(signed)
}

pub fn process_text_verify(input: &str, key: &str, format: TextSignFormat,sig:&str) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Verifier::load(key)?;
            signer.verify(&mut reader, &sig)?
        }
    };
    Ok(verified)
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    let key = match format {
        TextSignFormat::Blake3 => Blake3::generate()?,
        TextSignFormat::Ed25519 => Ed25519Signer::generate()?,
    };
    Ok(key)

}

impl TestSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).as_bytes().to_vec())
    }
}

impl TestSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(self.key.sign(&buffer).to_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        Ok(hash.as_bytes().to_vec() == sig)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let signature = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buffer, &signature).is_ok())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = [0u8;32];
        let key = key.to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer{
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk,pk])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3 { key };
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blanke3.txt")?;
        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..]).unwrap();
        assert!(blake3.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"hello world";
        let sig = sk.sign(&mut &data[..]).unwrap();
        assert!(pk.verify(&mut &data[..], &sig)?);
        Ok(())
    }
}