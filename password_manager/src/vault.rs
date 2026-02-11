use std::fs;
use std::path::PathBuf;

use zeroize::Zeroizing;

use crate::crypto::{decrypt, derive_key, encrypt, generate_salt};
use crate::errors::{PasswordManagerError, Result};
use crate::models::{Credential, EncryptedVault, Vault};

const VAULT_VERSION: u32 = 1;

/// VaultManager handles all vault operations
pub struct VaultManager {
    vault_path: PathBuf,
    vault: Option<Vault>,
    master_key: Option<Zeroizing<[u8; 32]>>,
}

impl VaultManager {
    /// Create a new VaultManager with the specified vault path
    pub fn new(vault_path: PathBuf) -> Self {
        Self {
            vault_path,
            vault: None,
            master_key: None,
        }
    }

    /// Get the default vault path based on OS
    pub fn default_vault_path() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir().ok_or_else(|| {
            PasswordManagerError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find local data directory",
            ))
        })?;

        let vault_dir = data_dir.join("password_manager");
        Ok(vault_dir.join("vault.enc"))
    }

    /// Check if vault exists
    pub fn vault_exists(&self) -> bool {
        self.vault_path.exists()
    }

    /// Initialize a new vault with a master password
    pub fn initialize(&mut self, master_password: &str) -> Result<()> {
        if self.vault_exists() {
            return Err(PasswordManagerError::VaultAlreadyExists(
                self.vault_path.display().to_string(),
            ));
        }

        // Create parent directories if they don't exist
        if let Some(parent) = self.vault_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Generate salt for key derivation
        let salt = generate_salt();

        // Derive encryption key from master password
        let key = derive_key(master_password, &salt)?;

        // Create empty vault
        let vault = Vault::new();

        // Save the vault
        self.master_key = Some(key);
        self.vault = Some(vault);
        self.save(&salt)?;

        Ok(())
    }

    /// Unlock the vault with master password
    pub fn unlock(&mut self, master_password: &str) -> Result<()> {
        if !self.vault_exists() {
            return Err(PasswordManagerError::VaultNotFound);
        }

        // Read encrypted vault from disk
        let encrypted_data = fs::read(&self.vault_path)?;
        let encrypted_vault: EncryptedVault = serde_json::from_slice(&encrypted_data)?;

        // Derive key from master password and stored salt
        let key = derive_key(master_password, &encrypted_vault.salt)?;

        // Decrypt vault data
        let decrypted_data = decrypt(&encrypted_vault.ciphertext, &key, &encrypted_vault.nonce)?;

        // Deserialize vault
        let vault: Vault = serde_json::from_slice(&decrypted_data)
            .map_err(|_| PasswordManagerError::InvalidMasterPassword)?;

        self.vault = Some(vault);
        self.master_key = Some(key);

        Ok(())
    }

    /// Save the vault to disk (encrypted)
    fn save(&self, salt: &[u8]) -> Result<()> {
        let vault = self
            .vault
            .as_ref()
            .ok_or(PasswordManagerError::InvalidInput(
                "Vault not loaded".to_string(),
            ))?;

        let key = self
            .master_key
            .as_ref()
            .ok_or(PasswordManagerError::InvalidInput(
                "Master key not set".to_string(),
            ))?;

        // Serialize vault
        let vault_json = serde_json::to_vec(vault)?;

        // Encrypt vault data
        let (ciphertext, nonce) = encrypt(&vault_json, key)?;

        // Create encrypted vault structure
        let encrypted_vault = EncryptedVault {
            salt: salt.to_vec(),
            nonce,
            ciphertext,
            version: VAULT_VERSION,
        };

        // Serialize and write to disk
        let encrypted_data = serde_json::to_vec(&encrypted_vault)?;
        fs::write(&self.vault_path, encrypted_data)?;

        Ok(())
    }

    /// Ensure vault is unlocked
    fn ensure_unlocked(&self) -> Result<()> {
        if self.vault.is_none() {
            return Err(PasswordManagerError::InvalidInput(
                "Vault is locked. Unlock first.".to_string(),
            ));
        }
        Ok(())
    }

    /// Add a new credential
    pub fn add_credential(&mut self, credential: Credential) -> Result<()> {
        self.ensure_unlocked()?;

        let vault = self.vault.as_mut().unwrap();

        // Check if credential already exists
        if vault.get_credential(&credential.service).is_some() {
            return Err(PasswordManagerError::CredentialAlreadyExists(
                credential.service.clone(),
            ));
        }

        vault
            .add_credential(credential)
            .map_err(PasswordManagerError::InvalidInput)?;

        // Re-read salt from existing vault
        let encrypted_data = fs::read(&self.vault_path)?;
        let encrypted_vault: EncryptedVault = serde_json::from_slice(&encrypted_data)?;

        self.save(&encrypted_vault.salt)?;
        Ok(())
    }

    /// Get a credential by service name
    pub fn get_credential(&self, service: &str) -> Result<&Credential> {
        self.ensure_unlocked()?;

        let vault = self.vault.as_ref().unwrap();
        vault
            .get_credential(service)
            .ok_or_else(|| PasswordManagerError::CredentialNotFound(service.to_string()))
    }

    /// Update a credential's password
    pub fn update_credential(&mut self, service: &str, new_password: String) -> Result<()> {
        self.ensure_unlocked()?;

        let vault = self.vault.as_mut().unwrap();
        let credential = vault
            .get_credential_mut(service)
            .ok_or_else(|| PasswordManagerError::CredentialNotFound(service.to_string()))?;

        credential.update_password(new_password);

        // Re-read salt from existing vault
        let encrypted_data = fs::read(&self.vault_path)?;
        let encrypted_vault: EncryptedVault = serde_json::from_slice(&encrypted_data)?;

        self.save(&encrypted_vault.salt)?;
        Ok(())
    }

    /// Remove a credential
    pub fn remove_credential(&mut self, service: &str) -> Result<()> {
        self.ensure_unlocked()?;

        let vault = self.vault.as_mut().unwrap();
        vault
            .remove_credential(service)
            .map_err(PasswordManagerError::CredentialNotFound)?;

        // Re-read salt from existing vault
        let encrypted_data = fs::read(&self.vault_path)?;
        let encrypted_vault: EncryptedVault = serde_json::from_slice(&encrypted_data)?;

        self.save(&encrypted_vault.salt)?;
        Ok(())
    }

    /// Search credentials by query
    pub fn search(&self, query: &str) -> Result<Vec<&Credential>> {
        self.ensure_unlocked()?;

        let vault = self.vault.as_ref().unwrap();
        Ok(vault.search(query))
    }

    /// List all credentials
    pub fn list_all(&self) -> Result<&[Credential]> {
        self.ensure_unlocked()?;

        let vault = self.vault.as_ref().unwrap();
        Ok(vault.list_all())
    }

    /// Lock the vault (clear from memory)
    pub fn lock(&mut self) {
        self.vault = None;
        self.master_key = None;
    }
}

impl Drop for VaultManager {
    fn drop(&mut self) {
        // Ensure sensitive data is cleared on drop
        self.lock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_vault_lifecycle() {
        use std::fs;

        let temp_file = NamedTempFile::new().unwrap();
        let vault_path = temp_file.path().to_path_buf();

        // Remove the temp file so we can test initialization
        drop(temp_file);
        let _ = fs::remove_file(&vault_path);

        let mut manager = VaultManager::new(vault_path.clone());

        // Initialize
        manager.initialize("test_password").unwrap();
        assert!(vault_path.exists());

        // Lock and unlock
        manager.lock();
        manager.unlock("test_password").unwrap();

        // Add credential
        let cred = Credential::new(
            "github.com".to_string(),
            "user@example.com".to_string(),
            "password123".to_string(),
            None,
        );
        manager.add_credential(cred).unwrap();

        // Get credential
        let retrieved = manager.get_credential("github.com").unwrap();
        assert_eq!(retrieved.username, "user@example.com");

        // Clean up
        let _ = fs::remove_file(&vault_path);
    }
}
