use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Entry in password history
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct PasswordHistoryEntry {
    pub password: String,
    #[zeroize(skip)]
    pub changed_at: DateTime<Utc>,
}

/// A single credential entry in the password manager
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Credential {
    pub service: String,
    pub username: String,
    #[serde(skip)]
    #[serde(default)]
    pub password: String, // Will be encrypted in vault
    pub notes: Option<String>,
    pub url: Option<String>,
    pub tags: Vec<String>,
    pub favorite: bool,
    #[zeroize(skip)]
    pub created_at: DateTime<Utc>,
    #[zeroize(skip)]
    pub updated_at: DateTime<Utc>,
    #[zeroize(skip)]
    pub last_accessed: Option<DateTime<Utc>>,
    #[zeroize(skip)]
    pub custom_fields: HashMap<String, String>,
    pub password_history: Vec<PasswordHistoryEntry>,
    pub totp_secret: Option<String>,
}

impl Credential {
    #[allow(clippy::too_many_arguments)]
    pub fn new(service: String, username: String, password: String, notes: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            service,
            username,
            password,
            notes,
            url: None,
            tags: Vec::new(),
            favorite: false,
            created_at: now,
            updated_at: now,
            last_accessed: None,
            custom_fields: HashMap::new(),
            password_history: Vec::new(),
            totp_secret: None,
        }
    }

    pub fn update_password(&mut self, new_password: String) {
        // Save old password to history
        if !self.password.is_empty() {
            self.password_history.push(PasswordHistoryEntry {
                password: self.password.clone(),
                changed_at: Utc::now(),
            });

            // Keep only last 10 passwords in history
            if self.password_history.len() > 10 {
                self.password_history.remove(0);
            }
        }

        self.password = new_password;
        self.updated_at = Utc::now();
    }

    #[allow(dead_code)]
    pub fn mark_accessed(&mut self) {
        self.last_accessed = Some(Utc::now());
    }

    #[allow(dead_code)]
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    #[allow(dead_code)]
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let len_before = self.tags.len();
        self.tags.retain(|t| t != tag);
        if self.tags.len() < len_before {
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn toggle_favorite(&mut self) {
        self.favorite = !self.favorite;
        self.updated_at = Utc::now();
    }

    #[allow(dead_code)]
    pub fn password_age_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.updated_at).num_days()
    }

    #[allow(dead_code)]
    pub fn is_old(&self, threshold_days: i64) -> bool {
        self.password_age_days() > threshold_days
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

/// Vault settings and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSettings {
    pub auto_lock_timeout_minutes: Option<u32>,
    pub require_totp: bool,
    pub backup_enabled: bool,
    pub backup_count: usize,
}

impl Default for VaultSettings {
    fn default() -> Self {
        Self {
            auto_lock_timeout_minutes: Some(15),
            require_totp: false,
            backup_enabled: true,
            backup_count: 5,
        }
    }
}

/// Vault statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VaultStats {
    pub total_credentials: usize,
    pub total_accesses: usize,
    pub weak_passwords: usize,
    pub reused_passwords: usize,
    pub old_passwords: usize,
    pub last_backup: Option<DateTime<Utc>>,
}

/// In-memory decrypted vault
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vault {
    pub credentials: Vec<Credential>,
    pub settings: VaultSettings,
    pub stats: VaultStats,
    #[serde(default)]
    pub audit_log: Vec<AuditLogEntry>,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub service: Option<String>,
    pub success: bool,
}

impl Drop for Vault {
    fn drop(&mut self) {
        // Zeroize credentials on drop
        self.credentials.zeroize();
    }
}

impl Vault {
    pub fn new() -> Self {
        Self::default()
    }

    /// Log an operation to the audit log
    #[allow(dead_code)]
    pub fn log_operation(&mut self, operation: String, service: Option<String>, success: bool) {
        self.audit_log.push(AuditLogEntry {
            timestamp: Utc::now(),
            operation,
            service,
            success,
        });

        // Keep only last 1000 entries
        if self.audit_log.len() > 1000 {
            self.audit_log.remove(0);
        }
    }

    /// Update vault statistics
    #[allow(dead_code)]
    pub fn update_stats(&mut self) {
        self.stats.total_credentials = self.credentials.len();
        // Other stats will be calculated on-demand
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

    /// Get favorites
    #[allow(dead_code)]
    pub fn get_favorites(&self) -> Vec<&Credential> {
        self.credentials.iter().filter(|c| c.favorite).collect()
    }

    /// Get recently accessed credentials (last 10)
    #[allow(dead_code)]
    pub fn get_recent(&self) -> Vec<&Credential> {
        let mut creds: Vec<&Credential> = self
            .credentials
            .iter()
            .filter(|c| c.last_accessed.is_some())
            .collect();

        creds.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        creds.into_iter().take(10).collect()
    }

    /// Get credentials by tag
    #[allow(dead_code)]
    pub fn get_by_tag(&self, tag: &str) -> Vec<&Credential> {
        self.credentials
            .iter()
            .filter(|c| c.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Find reused passwords
    #[allow(dead_code)]
    pub fn find_reused_passwords(&self) -> HashMap<String, Vec<String>> {
        let mut password_map: HashMap<String, Vec<String>> = HashMap::new();

        for cred in &self.credentials {
            if !cred.password.is_empty() {
                password_map
                    .entry(cred.password.clone())
                    .or_default()
                    .push(cred.service.clone());
            }
        }

        // Filter to only reused passwords
        password_map
            .into_iter()
            .filter(|(_, services)| services.len() > 1)
            .collect()
    }

    /// Find old passwords (older than threshold_days)
    #[allow(dead_code)]
    pub fn find_old_passwords(&self, threshold_days: i64) -> Vec<&Credential> {
        self.credentials
            .iter()
            .filter(|c| c.is_old(threshold_days))
            .collect()
    }

    /// Get all unique tags
    #[allow(dead_code)]
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .credentials
            .iter()
            .flat_map(|c| c.tags.clone())
            .collect();
        tags.sort();
        tags.dedup();
        tags
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
