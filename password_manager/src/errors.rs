use thiserror::Error;

/// Custom error types for the password manager
#[derive(Error, Debug)]
pub enum PasswordManagerError {
    #[error("Vault not found. Initialize with 'init' command first.")]
    VaultNotFound,

    #[error("Vault already exists at {0}")]
    VaultAlreadyExists(String),

    #[error("Invalid master password")]
    InvalidMasterPassword,

    #[error("Credential not found: {0}")]
    CredentialNotFound(String),

    #[error("Credential already exists: {0}")]
    CredentialAlreadyExists(String),

    #[error("Encryption failed: {0}")]
    EncryptionError(String),

    #[error("Decryption failed: {0}")]
    DecryptionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),
}

pub type Result<T> = std::result::Result<T, PasswordManagerError>;
