# Architecture Overview

## Design Philosophy

The password manager is built with security-first principles:

1. **Memory Safety**: All sensitive data automatically zeroized on drop
2. **Defense in Depth**: Multiple layers of security (encryption, key derivation, secure memory handling)
3. **No Plaintext**: Passwords never stored in plaintext anywhere
4. **Minimal Trust**: Each component has a single, well-defined responsibility

## Module Structure

```
password_manager/
├── src/
│   ├── main.rs       # Entry point, command routing
│   ├── cli.rs        # User interaction, prompts, output formatting
│   ├── vault.rs      # Vault management, file I/O
│   ├── crypto.rs     # Cryptographic operations
│   ├── models.rs     # Data structures
│   └── errors.rs     # Error types
```

## Security Architecture

### Encryption Flow

1. **Master Password** → Argon2id (64MB, 3 iterations) → **256-bit Key**
2. **Vault Data** + **256-bit Key** → AES-256-GCM → **Ciphertext + Nonce**
3. **Ciphertext + Nonce + Salt** → JSON → **vault.enc file**

### Key Derivation Parameters

- **Algorithm**: Argon2id (resistant to GPU/ASIC attacks)
- **Memory**: 64 MB (prevents parallel attacks)
- **Iterations**: 3 (balances security and performance)
- **Parallelism**: 4 threads
- **Output**: 256-bit key for AES-256-GCM

### Memory Safety

- `Credential` implements `Zeroize` and `ZeroizeOnDrop`
- Passwords automatically cleared from memory when dropped
- Master key stored in `Zeroizing<[u8; 32]>` wrapper
- `Vault` manually zeroizes credentials in Drop implementation

## Data Flow

### Initialization
```
User Password → Argon2id → Key → Empty Vault → Encrypt → Save to disk
```

### Unlock
```
User Password → Argon2id → Key → Read vault.enc → Decrypt → Load credentials
```

### Add Credential
```
Input → Create Credential → Add to Vault → Serialize → Encrypt → Save
```

### Get Credential
```
Service Name → Search Vault → Copy to Clipboard (or Show)
```

## File Format

The vault file (`vault.enc`) contains:

```json
{
  "salt": [/* 32 bytes for Argon2 */],
  "nonce": [/* 12 bytes for AES-GCM */],
  "ciphertext": [/* Encrypted vault data */],
  "version": 1
}
```

Inside the encrypted vault:

```json
{
  "credentials": [
    {
      "service": "example.com",
      "username": "user@example.com",
      "password": "encrypted_in_outer_layer",
      "notes": "Optional notes",
      "created_at": "2025-02-10T12:00:00Z",
      "updated_at": "2025-02-10T12:00:00Z"
    }
  ]
}
```

## Security Guarantees

1. **Encryption**: AES-256-GCM provides authenticated encryption
2. **Key Derivation**: Argon2id protects against brute-force attacks
3. **Memory Safety**: Sensitive data automatically zeroized
4. **Integrity**: GCM authentication tag prevents tampering
5. **Randomness**: OS-level entropy via `OsRng`

## Threat Model

### Protected Against

- Offline password guessing (via Argon2id)
- Memory dumps (via zeroization)
- Vault tampering (via GCM authentication)
- Weak passwords (via strong key derivation)
- Plain-text storage attacks

### NOT Protected Against

- Keyloggers or screen capture (user responsibility)
- Compromised OS/kernel
- Physical access with memory forensics during runtime
- Side-channel attacks on the CPU
- User choosing weak master password

## Future Enhancements

1. **Auto-lock**: Vault locks after inactivity timer
2. **Password strength meter**: Check entropy of generated passwords
3. **Import/Export**: Backup and restore functionality
4. **Multi-vault support**: Separate vaults for different contexts
5. **TOTP support**: Time-based one-time passwords
6. **Browser integration**: Auto-fill via native messaging
