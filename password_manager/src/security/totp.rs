use rand::Rng;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};

/// Generate a TOTP code from a secret
pub fn generate_totp(secret: &str) -> Result<String, String> {
    // Decode base32 secret
    let secret_bytes = decode_base32(secret)?;

    // Generate TOTP code
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get system time: {}", e))?
        .as_secs();

    let code = totp_custom::<Sha1>(DEFAULT_STEP, 6, &secret_bytes, timestamp);

    Ok(format!("{:06}", code))
}

/// Generate a random TOTP secret (base32 encoded)
pub fn generate_totp_secret() -> String {
    let mut rng = rand::thread_rng();
    let secret: Vec<u8> = (0..20).map(|_| rng.gen::<u8>()).collect();
    encode_base32(&secret)
}

/// Verify a TOTP code against a secret
pub fn verify_totp(secret: &str, code: &str) -> Result<bool, String> {
    let expected = generate_totp(secret)?;
    Ok(expected == code)
}

/// Decode base32 string to bytes
fn decode_base32(input: &str) -> Result<Vec<u8>, String> {
    const BASE32_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let input = input.to_uppercase().replace(['=', ' ', '-'], "");
    let mut output = Vec::new();
    let mut bits = 0u32;
    let mut bit_count = 0;

    for c in input.chars() {
        let val = BASE32_ALPHABET
            .iter()
            .position(|&x| x == c as u8)
            .ok_or_else(|| format!("Invalid base32 character: {}", c))? as u32;

        bits = (bits << 5) | val;
        bit_count += 5;

        if bit_count >= 8 {
            output.push((bits >> (bit_count - 8)) as u8);
            bit_count -= 8;
            bits &= (1 << bit_count) - 1;
        }
    }

    Ok(output)
}

/// Encode bytes to base32 string
fn encode_base32(input: &[u8]) -> String {
    const BASE32_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let mut output = String::new();
    let mut bits = 0u32;
    let mut bit_count = 0;

    for &byte in input {
        bits = (bits << 8) | byte as u32;
        bit_count += 8;

        while bit_count >= 5 {
            let index = (bits >> (bit_count - 5)) & 0x1F;
            output.push(BASE32_ALPHABET[index as usize] as char);
            bit_count -= 5;
            bits &= (1 << bit_count) - 1;
        }
    }

    if bit_count > 0 {
        let index = (bits << (5 - bit_count)) & 0x1F;
        output.push(BASE32_ALPHABET[index as usize] as char);
    }

    // Pad to multiple of 8
    while !output.len().is_multiple_of(8) {
        output.push('=');
    }

    output
}

/// Format TOTP code with spaces for readability (XXX XXX)
pub fn format_totp_code(code: &str) -> String {
    if code.len() == 6 {
        format!("{} {}", &code[0..3], &code[3..6])
    } else {
        code.to_string()
    }
}

/// Generate a TOTP URI for QR code generation
pub fn generate_totp_uri(secret: &str, account: &str, issuer: &str) -> String {
    format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}",
        urlencoding::encode(issuer),
        urlencoding::encode(account),
        secret,
        urlencoding::encode(issuer)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_encode_decode() {
        let original = b"Hello World";
        let encoded = encode_base32(original);
        let decoded = decode_base32(&encoded).unwrap();
        assert_eq!(original.to_vec(), decoded);
    }

    #[test]
    fn test_generate_secret() {
        let secret1 = generate_totp_secret();
        let secret2 = generate_totp_secret();
        assert_ne!(secret1, secret2);
        assert!(secret1.len() > 0);
    }

    #[test]
    fn test_totp_generation() {
        let secret = generate_totp_secret();
        let code = generate_totp(&secret);
        assert!(code.is_ok());
        assert_eq!(code.unwrap().len(), 6);
    }

    #[test]
    fn test_totp_verification() {
        let secret = generate_totp_secret();
        let code = generate_totp(&secret).unwrap();
        assert!(verify_totp(&secret, &code).unwrap());
        assert!(!verify_totp(&secret, "000000").unwrap());
    }

    #[test]
    fn test_format_totp_code() {
        assert_eq!(format_totp_code("123456"), "123 456");
        assert_eq!(format_totp_code("12345"), "12345");
    }

    #[test]
    fn test_totp_uri() {
        let uri = generate_totp_uri("SECRET", "user@example.com", "MyApp");
        assert!(uri.starts_with("otpauth://totp/"));
        assert!(uri.contains("secret=SECRET"));
    }
}
