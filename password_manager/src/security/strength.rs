use zxcvbn::zxcvbn;

/// Password strength score (0-4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PasswordStrength {
    VeryWeak = 0,
    Weak = 1,
    Fair = 2,
    Strong = 3,
    VeryStrong = 4,
}

impl PasswordStrength {
    pub fn from_score(score: u8) -> Self {
        match score {
            0 => PasswordStrength::VeryWeak,
            1 => PasswordStrength::Weak,
            2 => PasswordStrength::Fair,
            3 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PasswordStrength::VeryWeak => "Very Weak",
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Fair => "Fair",
            PasswordStrength::Strong => "Strong",
            PasswordStrength::VeryStrong => "Very Strong",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            PasswordStrength::VeryWeak => "red",
            PasswordStrength::Weak => "yellow",
            PasswordStrength::Fair => "cyan",
            PasswordStrength::Strong => "green",
            PasswordStrength::VeryStrong => "bright green",
        }
    }

    pub fn is_weak(&self) -> bool {
        (*self as u8) < 3
    }
}

/// Password strength analysis result
#[derive(Debug, Clone)]
pub struct PasswordAnalysis {
    pub strength: PasswordStrength,
    pub score: u8,
    pub entropy: f64,
    pub crack_time_seconds: Option<f64>,
    pub crack_time_display: String,
    pub warning: Option<String>,
    pub suggestions: Vec<String>,
}

/// Analyze password strength using zxcvbn
pub fn analyze_password(password: &str, user_inputs: &[&str]) -> PasswordAnalysis {
    let result = zxcvbn(password, user_inputs);

    let strength = PasswordStrength::from_score(result.score() as u8);
    let entropy = result.guesses_log10() * std::f64::consts::LOG2_10; // Convert log10 to bits

    // Format crack time - zxcvbn 3.x has a different API
    let crack_time_display = format!(
        "{}",
        result.crack_times().offline_slow_hashing_1e4_per_second()
    );
    let crack_time_seconds = None; // zxcvbn 3.x doesn't expose raw seconds easily

    // Get feedback
    let feedback_opt = result.feedback();
    let warning = feedback_opt
        .and_then(|f| f.warning())
        .map(|w| w.to_string());

    let suggestions: Vec<String> = feedback_opt
        .map(|f| f.suggestions().iter().map(|s| format!("{:?}", s)).collect())
        .unwrap_or_default();

    PasswordAnalysis {
        strength,
        score: result.score() as u8,
        entropy,
        crack_time_seconds,
        crack_time_display,
        warning,
        suggestions,
    }
}

/// Format crack time in human-readable format
fn format_crack_time(seconds: f64) -> String {
    if seconds < 1.0 {
        "Instant".to_string()
    } else if seconds < 60.0 {
        format!("{:.0} seconds", seconds)
    } else if seconds < 3600.0 {
        format!("{:.0} minutes", seconds / 60.0)
    } else if seconds < 86400.0 {
        format!("{:.1} hours", seconds / 3600.0)
    } else if seconds < 2592000.0 {
        format!("{:.1} days", seconds / 86400.0)
    } else if seconds < 31536000.0 {
        format!("{:.1} months", seconds / 2592000.0)
    } else if seconds < 3153600000.0 {
        format!("{:.1} years", seconds / 31536000.0)
    } else {
        "Centuries".to_string()
    }
}

/// Quick check if password is weak (score < 3)
pub fn is_weak_password(password: &str) -> bool {
    let analysis = zxcvbn(password, &[]);
    (analysis.score() as u8) < 3
}

/// Calculate password entropy (Shannon entropy)
pub fn calculate_entropy(password: &str) -> f64 {
    use std::collections::HashMap;

    if password.is_empty() {
        return 0.0;
    }

    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in password.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let len = password.len() as f64;
    let mut entropy = 0.0;

    for &count in freq.values() {
        let prob = count as f64 / len;
        entropy -= prob * prob.log2();
    }

    entropy * len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_passwords() {
        assert!(is_weak_password("password"));
        assert!(is_weak_password("12345678"));
        assert!(is_weak_password("qwerty"));
    }

    #[test]
    fn test_strong_passwords() {
        assert!(!is_weak_password("Tr0ub4dor&3xK9#mP"));
        assert!(!is_weak_password("correct-horse-battery-staple"));
    }

    #[test]
    fn test_password_strength_ordering() {
        assert!(PasswordStrength::VeryWeak < PasswordStrength::Weak);
        assert!(PasswordStrength::Weak < PasswordStrength::Fair);
        assert!(PasswordStrength::Fair < PasswordStrength::Strong);
        assert!(PasswordStrength::Strong < PasswordStrength::VeryStrong);
    }

    #[test]
    fn test_entropy_calculation() {
        let entropy1 = calculate_entropy("aaaa");
        let entropy2 = calculate_entropy("abcd");
        assert!(entropy2 > entropy1);
    }

    #[test]
    fn test_password_analysis() {
        let analysis = analyze_password("password123", &["user", "email"]);
        assert!(analysis.strength.is_weak());
        assert!(analysis.score < 3);
    }
}
