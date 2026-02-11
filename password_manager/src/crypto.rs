use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroizing;

use crate::errors::{PasswordManagerError, Result};

/// Key derivation parameters for Argon2
const ARGON2_MEMORY: u32 = 65536; // 64 MB
const ARGON2_ITERATIONS: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;

/// AES-GCM nonce size (96 bits / 12 bytes)
pub const NONCE_SIZE: usize = 12;

/// Derive a 256-bit encryption key from master password using Argon2id
pub fn derive_key(password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; 32]>> {
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            ARGON2_MEMORY,
            ARGON2_ITERATIONS,
            ARGON2_PARALLELISM,
            Some(32),
        )
        .map_err(|e| PasswordManagerError::EncryptionError(e.to_string()))?,
    );

    let mut key = Zeroizing::new([0u8; 32]);

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut *key)
        .map_err(|e| PasswordManagerError::EncryptionError(e.to_string()))?;

    Ok(key)
}

/// Hash the master password for verification (not storage)
/// Returns (hash_string, salt) for storage
#[allow(dead_code)]
pub fn hash_master_password(password: &str) -> Result<(String, Vec<u8>)> {
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(ARGON2_MEMORY, ARGON2_ITERATIONS, ARGON2_PARALLELISM, None)
            .map_err(|e| PasswordManagerError::EncryptionError(e.to_string()))?,
    );

    let salt = SaltString::generate(&mut OsRng);
    let salt_str = salt.as_str();
    let salt_bytes = salt_str.as_bytes().to_vec();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| PasswordManagerError::EncryptionError(e.to_string()))?
        .to_string();

    Ok((hash, salt_bytes))
}

/// Verify the master password against stored hash
#[allow(dead_code)]
pub fn verify_master_password(password: &str, hash_str: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash_str)
        .map_err(|e| PasswordManagerError::DecryptionError(e.to_string()))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Encrypt data using AES-256-GCM
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>)> {
    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| PasswordManagerError::EncryptionError(e.to_string()))?;

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| PasswordManagerError::EncryptionError(e.to_string()))?;

    Ok((ciphertext, nonce_bytes.to_vec()))
}

/// Decrypt data using AES-256-GCM
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    if nonce.len() != NONCE_SIZE {
        return Err(PasswordManagerError::DecryptionError(
            "Invalid nonce size".to_string(),
        ));
    }

    let nonce = Nonce::from_slice(nonce);

    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| PasswordManagerError::DecryptionError(e.to_string()))?;

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| PasswordManagerError::DecryptionError(e.to_string()))?;

    Ok(plaintext)
}

/// Generate a cryptographically secure random password
pub fn generate_password(
    length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_numbers: bool,
    use_symbols: bool,
) -> String {
    let mut charset = Vec::new();

    if use_uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_lowercase {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }
    if use_numbers {
        charset.extend_from_slice(b"0123456789");
    }
    if use_symbols {
        charset.extend_from_slice(b"!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }

    let mut password = Vec::with_capacity(length);
    let mut rng = OsRng;

    for _ in 0..length {
        let idx = (rng.next_u32() as usize) % charset.len();
        password.push(charset[idx]);
    }

    String::from_utf8(password).expect("Valid UTF-8 password")
}

/// Generate random salt for key derivation
pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [42u8; 32];
        let plaintext = b"Hello, World!";

        let (ciphertext, nonce) = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_key_derivation() {
        let password = "test_password";
        let salt = generate_salt();

        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(password, &salt).unwrap();

        assert_eq!(*key1, *key2);
    }

    #[test]
    fn test_password_generation() {
        let password = generate_password(20, true, true, true, true);
        assert_eq!(password.len(), 20);
    }

    #[test]
    fn test_master_password_verification() {
        let password = "secure_master_pass";
        let (hash, _) = hash_master_password(password).unwrap();

        assert!(verify_master_password(password, &hash).unwrap());
        assert!(!verify_master_password("wrong_password", &hash).unwrap());
    }
}
