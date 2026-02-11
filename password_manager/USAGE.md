# Usage Guide

## Quick Start

### 1. Installation

```bash
# Build and install
make install

# Or use directly from target
make release
./target/release/password_manager --help
```

### 2. Initialize Your Vault

First time setup:

```bash
password_manager init
```

You'll be prompted to create a master password. Choose a strong password - this is the ONLY password you need to remember.

```
Initializing new password vault...

Enter master password: ********
Confirm master password: ********

✓ Vault initialized successfully!
Your vault is encrypted and stored securely.
```

## Common Operations

### Adding Credentials

#### Interactive Mode (Recommended)

```bash
password_manager add
```

The tool will prompt you for:
- Service name (e.g., github.com, gmail.com)
- Username/email
- Password (or generate one)
- Optional notes

#### With Generated Password

```bash
password_manager add --generate
```

Automatically generates a secure 24-character password.

#### Command-Line Arguments

```bash
password_manager add --service github.com --username myuser@email.com
```

### Retrieving Passwords

#### Copy to Clipboard (Default)

```bash
password_manager get github.com
```

Password is automatically copied to your clipboard.

```
Master password: ********
✓ Password for github.com copied to clipboard
```

#### Display on Screen

```bash
password_manager get github.com --show
```

Shows all credential details in the terminal.

### Listing Credentials

```bash
password_manager list
```

Shows all stored services:

```
Master password: ********

Stored Credentials:
────────────────────────────────────────────────
  • github.com (user@example.com)
  • gitlab.com (user@example.com)
  • gmail.com (myemail@gmail.com)
────────────────────────────────────────────────
3 credentials found
```

### Searching

```bash
password_manager search git
```

Finds all services matching "git":

```
Master password: ********

Search results for 'git':
────────────────────────────────────────────────
  • github.com (user@example.com)
  • gitlab.com (user@example.com)
────────────────────────────────────────────────
```

### Updating Passwords

```bash
password_manager update github.com
```

Prompts for new password, or:

```bash
password_manager update github.com --generate
```

Generates a new random password.

### Removing Credentials

```bash
password_manager remove github.com
```

Asks for confirmation before deleting.

### Generating Passwords

Generate a password without storing it:

```bash
# Default: 24 characters, all character types
password_manager generate

# Custom length
password_manager generate --length 32

# Exclude certain character types
password_manager generate --no-symbols
password_manager generate --length 16 --no-uppercase --no-symbols
```

Output:

```
Generated Password:
────────────────────────────────────────────────
  xK9$mP2#nQ8*vL4@zR7!wT5%
────────────────────────────────────────────────
Length: 24 characters
✓ Password copied to clipboard
```

## Advanced Usage

### Vault Location

Default vault locations:
- **Linux**: `~/.local/share/password_manager/vault.enc`
- **macOS**: `~/Library/Application Support/password_manager/vault.enc`
- **Windows**: `%APPDATA%\password_manager\vault.enc`

### Backing Up Your Vault

```bash
# Find your vault
ls ~/.local/share/password_manager/

# Copy to backup location
cp ~/.local/share/password_manager/vault.enc ~/backups/vault-backup-2025-02-10.enc
```

The vault file is encrypted, so it's safe to store in cloud storage or send via email.

### Restoring from Backup

```bash
cp ~/backups/vault-backup-2025-02-10.enc ~/.local/share/password_manager/vault.enc
```

### Resetting/Starting Over

```bash
rm ~/.local/share/password_manager/vault.enc
password_manager init
```

## Tips and Best Practices

### Master Password

- Use a passphrase (4+ random words) or 16+ character password
- Never write it down in plaintext
- Don't use it for any other service
- Example good passphrase: "correct-horse-battery-staple-2025"

### Generated Passwords

- Always use the default 24-character length for important accounts
- Include all character types (uppercase, lowercase, numbers, symbols)
- Don't reduce security for "convenience"

### Workflow

1. Generate password when creating new account
2. Add to vault immediately
3. Use "get" command to retrieve later
4. Update passwords regularly (every 3-6 months)

### Security

- Clear clipboard after using passwords
- Lock your screen when away from computer
- Don't share your vault file's master password
- Keep regular encrypted backups
- Never commit vault file to git

## Troubleshooting

### "Vault not found" error

Initialize the vault first:
```bash
password_manager init
```

### "Invalid master password" error

Double-check your password. If forgotten, you'll need to reset (lose all data):
```bash
rm ~/.local/share/password_manager/vault.enc
password_manager init
```

### Clipboard not working

Use `--show` flag to display password in terminal:
```bash
password_manager get service --show
```

## Examples

### Complete Workflow

```bash
# 1. Initialize
password_manager init

# 2. Add credentials with generated password
password_manager add --service github.com --username myuser --generate

# 3. Add credential with manual password
password_manager add
# Enter: gmail.com, myemail@gmail.com, [password], "Personal email"

# 4. List all
password_manager list

# 5. Get password (copy to clipboard)
password_manager get github.com

# 6. Search
password_manager search mail

# 7. Update with new generated password
password_manager update github.com --generate

# 8. Remove old credential
password_manager remove oldservice.com
```

### Daily Usage

```bash
# Morning: check what accounts you have
password_manager list

# Need to login somewhere
password_manager get amazon.com

# Creating new account
password_manager generate  # Copy password to new account
password_manager add --service newsite.com --username myuser
```

## Getting Help

```bash
# General help
password_manager --help

# Command-specific help
password_manager add --help
password_manager generate --help
```
