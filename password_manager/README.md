# Password Manager

A secure, production-ready CLI password manager written in Rust.

## Features

- **Strong Encryption**: AES-256-GCM encryption for vault storage
- **Secure Key Derivation**: Argon2 for master password hashing
- **Memory Safety**: Automatic zeroization of sensitive data
- **Password Generator**: Cryptographically secure random password generation
- **Clipboard Integration**: Copy passwords to clipboard safely
- **Search**: Quick search through stored credentials
- **Auto-lock**: Vault locks after inactivity

## Security

- All passwords encrypted with AES-256-GCM
- Master password hashed with Argon2id
- Sensitive memory automatically zeroized
- No plaintext passwords ever written to disk
- Constant-time comparisons for authentication

## Installation

```bash
make install
```

Or build from source:

```bash
make release
# Binary will be at: target/release/password_manager
```

## Usage

```bash
# Initialize vault (first time)
password_manager init

# Add a new credential
password_manager add

# List all credentials
password_manager list

# Search for credentials
password_manager search <service>

# Get a password (copy to clipboard)
password_manager get <service>

# Generate a secure password
password_manager generate

# Update existing credential
password_manager update <service>

# Remove a credential
password_manager remove <service>
```

## Development

```bash
make help        # Show all available commands
make build       # Build in debug mode
make test        # Run tests
make fmt         # Format code
make clippy      # Run linter
make audit       # Security audit
```

## Vault Location

By default, the encrypted vault is stored at:
- Linux/macOS: `~/.local/share/password_manager/vault.enc`
- Windows: `%APPDATA%\password_manager\vault.enc`

## License

MIT
