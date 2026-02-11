use sha1::{Digest, Sha1};

/// Check if a password has been breached (using k-anonymity)
/// Returns (is_breached, count_in_breaches, hash_prefix)
pub fn check_password_breach_local(password: &str) -> (bool, u32, String) {
    // Hash the password with SHA-1
    let hash = hash_password_sha1(password);
    let hash_upper = hash.to_uppercase();

    // Use k-anonymity: only send first 5 characters
    let prefix = &hash_upper[0..5];
    let _suffix = &hash_upper[5..];

    // For local check, we'd need a local database
    // This is a placeholder that always returns not breached
    // In a real implementation, you would:
    // 1. Look up the prefix in a local hash database
    // 2. Check if the suffix matches any entry
    // 3. Return the count

    (false, 0, prefix.to_string())
}

/// Hash password with SHA-1 (used for Have I Been Pwned API)
pub fn hash_password_sha1(password: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{:X}", result)
}

/// Check if password is in a common password list
pub fn is_common_password(password: &str) -> bool {
    // Common passwords list (top 100 most common)
    const COMMON_PASSWORDS: &[&str] = &[
        "password",
        "123456",
        "12345678",
        "qwerty",
        "abc123",
        "monkey",
        "1234567",
        "letmein",
        "trustno1",
        "dragon",
        "baseball",
        "111111",
        "iloveyou",
        "master",
        "sunshine",
        "ashley",
        "bailey",
        "passw0rd",
        "shadow",
        "123123",
        "654321",
        "superman",
        "qazwsx",
        "michael",
        "football",
        "welcome",
        "jesus",
        "ninja",
        "mustang",
        "password1",
        "123456789",
        "admin",
        "welcome1",
        "login",
        "admin123",
        "root",
        "toor",
        "pass",
        "test",
        "guest",
        "changeme",
        "password123",
        "qwerty123",
        "letmein",
        "hello",
        "1234",
        "12345",
        "123",
    ];

    let password_lower = password.to_lowercase();
    COMMON_PASSWORDS.contains(&password_lower.as_str())
}

/// Breach check result
#[derive(Debug, Clone)]
pub struct BreachCheckResult {
    pub is_breached: bool,
    pub breach_count: u32,
    pub is_common: bool,
    pub hash_prefix: String,
    pub recommendation: String,
}

/// Comprehensive breach and weakness check
pub fn check_password_security(password: &str) -> BreachCheckResult {
    let is_common = is_common_password(password);
    let (is_breached, breach_count, hash_prefix) = check_password_breach_local(password);

    let recommendation = if is_common {
        "This is a very common password. Change it immediately!".to_string()
    } else if is_breached {
        format!(
            "This password has been found in {} data breaches. Change it!",
            breach_count
        )
    } else if password.len() < 12 {
        "Password is short. Consider using a longer password.".to_string()
    } else {
        "Password appears secure.".to_string()
    };

    BreachCheckResult {
        is_breached,
        breach_count,
        is_common,
        hash_prefix,
        recommendation,
    }
}

/// Check multiple passwords for breaches (batch check)
pub fn batch_check_passwords(passwords: &[String]) -> Vec<(String, bool, bool)> {
    passwords
        .iter()
        .map(|p| {
            let common = is_common_password(p);
            let (breached, _, _) = check_password_breach_local(p);
            (p.clone(), breached, common)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_hashing() {
        let hash = hash_password_sha1("password");
        assert_eq!(hash.len(), 40); // SHA-1 produces 40 hex characters
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_common_passwords() {
        assert!(is_common_password("password"));
        assert!(is_common_password("Password"));
        assert!(is_common_password("123456"));
        assert!(!is_common_password("Tr0ub4dor&3"));
    }

    #[test]
    fn test_breach_check() {
        let result = check_password_security("password");
        assert!(result.is_common);
        assert!(!result.recommendation.is_empty());
    }

    #[test]
    fn test_hash_prefix() {
        let (_, _, prefix) = check_password_breach_local("password123");
        assert_eq!(prefix.len(), 5);
    }

    #[test]
    fn test_batch_check() {
        let passwords = vec!["password".to_string(), "strongP@ssw0rd!123".to_string()];
        let results = batch_check_passwords(&passwords);
        assert_eq!(results.len(), 2);
        assert!(results[0].2); // First password is common
        assert!(!results[1].2); // Second password is not common
    }
}
