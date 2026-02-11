use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A single credential entry in the password manager
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Credential {
    pub service: String,
    pub username: String,
    #[serde(skip)]
    #[serde(default)]
    pub password: String, // Will be encrypted in vault
    pub notes: Option<String>,
    #[zeroize(skip)]
    pub created_at: DateTime<Utc>,
    #[zeroize(skip)]
    pub updated_at: DateTime<Utc>,
}

impl Credential {
    pub fn new(service: String, username: String, password: String, notes: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            service,
            username,
            password,
            notes,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_password(&mut self, new_password: String) {
        self.password = new_password;
        self.updated_at = Utc::now();
    }
}

/// Encrypted vault data stored on disk
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedVault {
    /// Salt used for key derivation (Argon2)
    pub salt: Vec<u8>,
    /// Nonce for AES-GCM encryption
    pub nonce: Vec<u8>,
    /// Encrypted credential data
    pub ciphertext: Vec<u8>,
    /// Version for future compatibility
    pub version: u32,
}

/// In-memory decrypted vault
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Vault {
    pub credentials: Vec<Credential>,
}

impl Drop for Vault {
    fn drop(&mut self) {
        // Zeroize credentials on drop
        self.credentials.zeroize();
    }
}

impl Vault {
    pub fn new() -> Self {
        Self {
            credentials: Vec::new(),
        }
    }

    pub fn add_credential(&mut self, credential: Credential) -> Result<(), String> {
        if self
            .credentials
            .iter()
            .any(|c| c.service == credential.service)
        {
            return Err(format!("Credential already exists: {}", credential.service));
        }
        self.credentials.push(credential);
        Ok(())
    }

    pub fn get_credential(&self, service: &str) -> Option<&Credential> {
        self.credentials.iter().find(|c| c.service == service)
    }

    pub fn get_credential_mut(&mut self, service: &str) -> Option<&mut Credential> {
        self.credentials.iter_mut().find(|c| c.service == service)
    }

    pub fn remove_credential(&mut self, service: &str) -> Result<(), String> {
        let initial_len = self.credentials.len();
        self.credentials.retain(|c| c.service != service);
        if self.credentials.len() == initial_len {
            return Err(format!("Credential not found: {}", service));
        }
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<&Credential> {
        let query_lower = query.to_lowercase();
        self.credentials
            .iter()
            .filter(|c| {
                c.service.to_lowercase().contains(&query_lower)
                    || c.username.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn list_all(&self) -> &[Credential] {
        &self.credentials
    }
}

/// Configuration for password generation
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 24,
            use_uppercase: true,
            use_lowercase: true,
            use_numbers: true,
            use_symbols: true,
        }
    }
}
