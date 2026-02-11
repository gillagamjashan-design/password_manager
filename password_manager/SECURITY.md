# Security Considerations

## Overview

This password manager follows security best practices for handling sensitive data in Rust applications.

## Cryptographic Choices

### AES-256-GCM

- **Algorithm**: AES with 256-bit keys in Galois/Counter Mode
- **Why**: Industry standard, hardware-accelerated, provides both confidentiality and integrity
- **Key Size**: 256 bits (128-bit security level)
- **Nonce**: 96 bits, randomly generated per encryption
- **Authentication**: Built-in authentication tag prevents tampering

### Argon2id

- **Why Argon2id**: Winner of Password Hashing Competition, resistant to:
  - GPU cracking attempts
  - ASIC-based attacks
  - Side-channel attacks (hybrid of Argon2i and Argon2d)
- **Parameters**:
  - Memory: 64 MB (prevents parallel GPU attacks)
  - Iterations: 3 (time cost)
  - Parallelism: 4 threads
  - Salt: 32 bytes, randomly generated

### Random Number Generation

- Uses `OsRng` from `rand` crate
- Provides cryptographically secure random numbers from OS
- Used for: nonces, salts, password generation

## Memory Safety

### Zeroization

All sensitive data is automatically cleared from memory:

```rust
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Credential {
    pub password: String,  // Automatically zeroized on drop
    // ...
}
```

### Key Storage

- Master key stored in `Zeroizing<[u8; 32]>` wrapper
- Automatically cleared when `VaultManager` is dropped
- Never logged or printed

### String Handling

- Passwords read via `rpassword` (no echo to terminal)
- Clipboard contents replaced after copy
- No sensitive data in error messages

## Implementation Details

### Constant-Time Operations

While not explicitly implemented, the cryptographic libraries used (`aes-gcm`, `argon2`) implement constant-time operations internally to prevent timing attacks.

### Salt Management

- Each vault gets a unique 32-byte salt
- Salt stored alongside encrypted data (not secret)
- Same salt used for all encryptions within a vault (key derivation only)

### Nonce Management

- Fresh 96-bit nonce for each encryption operation
- Never reused with the same key
- Stored with ciphertext

## Security Practices

### What We Do

1. ✅ Use well-audited cryptographic libraries
2. ✅ Zeroize sensitive memory
3. ✅ Generate cryptographically secure random values
4. ✅ Use authenticated encryption (prevents tampering)
5. ✅ Strong key derivation (prevents brute force)
6. ✅ No plaintext password storage
7. ✅ Minimal dependencies (reduces attack surface)

### What We Don't Do

1. ❌ Store passwords in plaintext
2. ❌ Log sensitive information
3. ❌ Use weak encryption (MD5, SHA1, etc.)
4. ❌ Reuse nonces
5. ❌ Store keys unencrypted

## Known Limitations

1. **No Network Security**: This is a local-only tool
2. **Terminal Security**: Cannot prevent terminal screen capture
3. **Process Memory**: Running process memory could be dumped
4. **Master Password**: Security depends entirely on master password strength
5. **Clipboard**: Password temporarily in clipboard (system-dependent security)

## Best Practices for Users

1. **Master Password**:
   - Use a strong, unique master password
   - Never share or write down
   - Consider using a passphrase (e.g., "correct-horse-battery-staple")

2. **Generated Passwords**:
   - Use default settings (24 chars, all character types)
   - Don't reduce length below 16 characters
   - Enable all character types for maximum entropy

3. **Vault Security**:
   - Keep vault file backed up (encrypted)
   - Don't commit vault file to version control
   - Lock vault when done (close the application)

4. **System Security**:
   - Keep OS and security updates current
   - Use full-disk encryption
   - Be aware of keyloggers and screen recorders
   - Clear clipboard after use

## Vulnerability Reporting

If you discover a security vulnerability, please:

1. **Do NOT** create a public GitHub issue
2. Email details to [your-security-email]
3. Allow reasonable time for fix before disclosure

## Security Audit Status

This project has NOT been professionally audited. Use at your own risk.

For production use, consider:
- Professional security audit
- Formal threat modeling
- Penetration testing
- Code review by security experts

## Compliance

This implementation follows guidelines from:
- OWASP Password Storage Cheat Sheet
- NIST SP 800-63B (Digital Identity Guidelines)
- Argon2 RFC 9106

## License and Disclaimer

This software is provided "as is" without warranty. The authors assume no liability for data loss or security breaches. Always maintain backups of important data.
