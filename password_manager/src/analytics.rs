#[allow(unused_imports)]
use crate::models::{Credential, Vault};
use crate::security::{analyze_password, is_common_password, is_weak_password, PasswordStrength};
use std::collections::HashMap;

/// Vault health score (0-100)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VaultHealth {
    pub overall_score: u32,
    pub total_credentials: usize,
    pub weak_passwords: usize,
    pub reused_passwords: usize,
    pub old_passwords: usize,
    pub strong_passwords: usize,
    pub common_passwords: usize,
    pub with_totp: usize,
    pub average_password_age_days: f64,
    pub recommendations: Vec<String>,
}

impl VaultHealth {
    #[allow(dead_code)]
    pub fn score_category(&self) -> &'static str {
        match self.overall_score {
            0..=20 => "Critical",
            21..=40 => "Poor",
            41..=60 => "Fair",
            61..=80 => "Good",
            81..=100 => "Excellent",
            _ => "Unknown",
        }
    }

    #[allow(dead_code)]
    pub fn score_color(&self) -> &'static str {
        match self.overall_score {
            0..=20 => "red",
            21..=40 => "yellow",
            41..=60 => "cyan",
            61..=80 => "green",
            81..=100 => "bright green",
            _ => "white",
        }
    }
}

/// Analyze vault health
#[allow(dead_code)]
pub fn analyze_vault_health(vault: &Vault, old_password_threshold_days: i64) -> VaultHealth {
    let total_credentials = vault.credentials.len();

    if total_credentials == 0 {
        return VaultHealth {
            overall_score: 100,
            total_credentials: 0,
            weak_passwords: 0,
            reused_passwords: 0,
            old_passwords: 0,
            strong_passwords: 0,
            common_passwords: 0,
            with_totp: 0,
            average_password_age_days: 0.0,
            recommendations: vec![
                "Add credentials to start using the password manager.".to_string()
            ],
        };
    }

    // Analyze each password
    let mut weak_passwords = 0;
    let mut strong_passwords = 0;
    let mut common_passwords = 0;
    let mut with_totp = 0;
    let mut total_age_days = 0i64;

    for cred in &vault.credentials {
        // Check strength
        if is_weak_password(&cred.password) {
            weak_passwords += 1;
        } else {
            let analysis = analyze_password(&cred.password, &[&cred.service, &cred.username]);
            if analysis.strength >= PasswordStrength::Strong {
                strong_passwords += 1;
            }
        }

        // Check if common
        if is_common_password(&cred.password) {
            common_passwords += 1;
        }

        // Check TOTP
        if cred.totp_secret.is_some() {
            with_totp += 1;
        }

        // Add to age
        total_age_days += cred.password_age_days();
    }

    // Find reused passwords
    let reused_map = vault.find_reused_passwords();
    let reused_passwords = reused_map.len();

    // Find old passwords
    let old_passwords = vault.find_old_passwords(old_password_threshold_days).len();

    // Calculate average age
    let average_password_age_days = if total_credentials > 0 {
        total_age_days as f64 / total_credentials as f64
    } else {
        0.0
    };

    // Calculate score (0-100)
    let mut score = 100u32;

    // Penalties
    let weak_penalty = (weak_passwords as f64 / total_credentials as f64 * 30.0) as u32;
    let reused_penalty = (reused_passwords as f64 / total_credentials as f64 * 25.0) as u32;
    let old_penalty = (old_passwords as f64 / total_credentials as f64 * 20.0) as u32;
    let common_penalty = (common_passwords as f64 / total_credentials as f64 * 15.0) as u32;

    score = score.saturating_sub(weak_penalty);
    score = score.saturating_sub(reused_penalty);
    score = score.saturating_sub(old_penalty);
    score = score.saturating_sub(common_penalty);

    // Bonus for TOTP
    let totp_bonus = (with_totp as f64 / total_credentials as f64 * 10.0) as u32;
    score = score.saturating_add(totp_bonus).min(100);

    // Generate recommendations
    let mut recommendations = Vec::new();

    if weak_passwords > 0 {
        recommendations.push(format!(
            "Update {} weak password(s) to stronger alternatives.",
            weak_passwords
        ));
    }

    if reused_passwords > 0 {
        recommendations.push(format!(
            "Change {} reused password(s). Each credential should have a unique password.",
            reused_passwords
        ));
    }

    if old_passwords > 0 {
        recommendations.push(format!(
            "Update {} old password(s) (older than {} days).",
            old_passwords, old_password_threshold_days
        ));
    }

    if common_passwords > 0 {
        recommendations.push(format!(
            "Replace {} common password(s) immediately!",
            common_passwords
        ));
    }

    if with_totp < total_credentials / 2 {
        recommendations.push("Enable 2FA/TOTP for more accounts to improve security.".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("Your vault is in excellent condition! Keep it up.".to_string());
    }

    VaultHealth {
        overall_score: score,
        total_credentials,
        weak_passwords,
        reused_passwords,
        old_passwords,
        strong_passwords,
        common_passwords,
        with_totp,
        average_password_age_days,
        recommendations,
    }
}

/// Detailed password report for a single credential
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PasswordReport {
    pub service: String,
    pub strength: PasswordStrength,
    pub is_weak: bool,
    pub is_common: bool,
    pub is_reused: bool,
    pub is_old: bool,
    pub age_days: i64,
    pub has_totp: bool,
    pub warnings: Vec<String>,
}

/// Generate detailed reports for all credentials
#[allow(dead_code)]
pub fn generate_password_reports(vault: &Vault, old_threshold_days: i64) -> Vec<PasswordReport> {
    let reused_map = vault.find_reused_passwords();

    let mut reused_passwords: HashMap<String, bool> = HashMap::new();
    for (password, services) in reused_map {
        if services.len() > 1 {
            reused_passwords.insert(password, true);
        }
    }

    vault
        .credentials
        .iter()
        .map(|cred| {
            let is_weak = is_weak_password(&cred.password);
            let is_common = is_common_password(&cred.password);
            let is_reused = reused_passwords.contains_key(&cred.password);
            let is_old = cred.is_old(old_threshold_days);
            let age_days = cred.password_age_days();
            let has_totp = cred.totp_secret.is_some();

            let analysis = analyze_password(&cred.password, &[&cred.service, &cred.username]);
            let strength = analysis.strength;

            let mut warnings = Vec::new();
            if is_common {
                warnings.push("Common password - change immediately!".to_string());
            }
            if is_weak {
                warnings.push("Weak password strength".to_string());
            }
            if is_reused {
                warnings.push("Password reused across services".to_string());
            }
            if is_old {
                warnings.push(format!("Password is {} days old", age_days));
            }
            if !has_totp {
                warnings.push("Consider enabling 2FA/TOTP".to_string());
            }

            PasswordReport {
                service: cred.service.clone(),
                strength,
                is_weak,
                is_common,
                is_reused,
                is_old,
                age_days,
                has_totp,
                warnings,
            }
        })
        .collect()
}

/// Find credentials that need attention
#[allow(dead_code)]
pub fn find_credentials_needing_attention(vault: &Vault, old_threshold_days: i64) -> Vec<String> {
    let reports = generate_password_reports(vault, old_threshold_days);

    reports
        .into_iter()
        .filter(|r| r.is_weak || r.is_common || r.is_reused || r.is_old)
        .map(|r| r.service)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Credential;

    #[test]
    fn test_empty_vault_health() {
        let vault = Vault::new();
        let health = analyze_vault_health(&vault, 90);
        assert_eq!(health.overall_score, 100);
        assert_eq!(health.total_credentials, 0);
    }

    #[test]
    fn test_vault_with_weak_passwords() {
        let mut vault = Vault::new();
        vault
            .add_credential(Credential::new(
                "test1".to_string(),
                "user".to_string(),
                "password".to_string(),
                None,
            ))
            .unwrap();

        let health = analyze_vault_health(&vault, 90);
        assert!(health.weak_passwords > 0);
        assert!(health.overall_score < 100);
    }

    #[test]
    fn test_health_score_categories() {
        let health = VaultHealth {
            overall_score: 95,
            total_credentials: 10,
            weak_passwords: 0,
            reused_passwords: 0,
            old_passwords: 0,
            strong_passwords: 10,
            common_passwords: 0,
            with_totp: 5,
            average_password_age_days: 30.0,
            recommendations: vec![],
        };

        assert_eq!(health.score_category(), "Excellent");
    }

    #[test]
    fn test_password_reports() {
        let mut vault = Vault::new();
        vault
            .add_credential(Credential::new(
                "test".to_string(),
                "user".to_string(),
                "password123".to_string(),
                None,
            ))
            .unwrap();

        let reports = generate_password_reports(&vault, 90);
        assert_eq!(reports.len(), 1);
        assert!(reports[0].is_weak || reports[0].warnings.len() > 0);
    }
}
