use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

#[derive(Debug, Clone, Zeroize, Serialize, Deserialize)]
pub struct EncryptedPassword {
    nonce: String,
    ciphertext: String,
}

impl EncryptedPassword {
    pub fn encrypt(plaintext: &str, key: &[u8; KEY_LENGTH]) -> Result<Self, SecurityError> {
        let mut nonce_bytes = [0u8; NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| SecurityError::Encryption(e.to_string()))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| SecurityError::Encryption(e.to_string()))?;

        Ok(Self {
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
            ciphertext: general_purpose::STANDARD.encode(ciphertext),
        })
    }

    pub fn decrypt(&self, key: &[u8; KEY_LENGTH]) -> Result<String, SecurityError> {
        let nonce_bytes = general_purpose::STANDARD
            .decode(&self.nonce)
            .map_err(|e| SecurityError::Decoding(e.to_string()))?;
        let ciphertext = general_purpose::STANDARD
            .decode(&self.ciphertext)
            .map_err(|e| SecurityError::Decoding(e.to_string()))?;

        let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| SecurityError::Decryption(e.to_string()))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext_bytes = cipher
            .decrypt(nonce, ciphertext.as_slice())
            .map_err(|e| SecurityError::Decryption(e.to_string()))?;

        String::from_utf8(plaintext_bytes).map_err(|e| SecurityError::Utf8(e.to_string()))
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn ciphertext(&self) -> &str {
        &self.ciphertext
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Decryption error: {0}")]
    Decryption(String),
    #[error("Decoding error: {0}")]
    Decoding(String),
    #[error("UTF-8 error: {0}")]
    Utf8(String),
}

fn get_master_key() -> [u8; KEY_LENGTH] {
    let mut key = [0u8; KEY_LENGTH];
    let app_name = env!("CARGO_PKG_NAME");
    let key_material = format!("{}-master-key-v1", app_name);
    let hash = blake3::hash(key_material.as_bytes());
    key.copy_from_slice(&hash.as_bytes()[..KEY_LENGTH]);
    key
}

pub fn encrypt_password(plaintext: &str) -> Result<EncryptedPassword, SecurityError> {
    let master_key = get_master_key();
    EncryptedPassword::encrypt(plaintext, &master_key)
}

pub fn decrypt_password(encrypted: &EncryptedPassword) -> Result<String, SecurityError> {
    let master_key = get_master_key();
    encrypted.decrypt(&master_key)
}
