# Password Manager Expansion Plan

## Current State Analysis

**Existing Features:**
- AES-256-GCM encryption with Argon2id key derivation
- Basic CRUD operations (add, get, update, remove, list)
- Simple search functionality
- Password generation
- CLI interface with clap

**Code Quality:**
- Well-structured modules (crypto, vault, models, cli, errors)
- Memory safety with zeroize
- Unit tests for crypto operations
- ~1,233 lines of Rust code

## Expansion Strategy (Phased Approach)

### Phase 1: Enhanced Data Models & Core Infrastructure
**Priority: HIGH | Risk: LOW | Complexity: MEDIUM**

1. **Enhanced Credential Model**
   - Add categories/tags (Vec<String>)
   - Add last_accessed timestamp
   - Add favorite flag
   - Add URL field
   - Add custom fields (HashMap)
   - Add password history (Vec of previous passwords with timestamps)

2. **Vault Metadata**
   - Add vault statistics tracking
   - Add access audit log
   - Add vault settings (auto-lock timeout, etc.)

3. **Multi-Vault Support**
   - Refactor VaultManager to support multiple vault profiles
   - Add vault switching mechanism
   - Vault profiles (personal/work/etc.)

**Estimated Work:** 4-6 features | 300-400 LOC

---

### Phase 2: Security Upgrades
**Priority: CRITICAL | Risk: MEDIUM | Complexity: HIGH**

1. **TOTP Support**
   - Add totp-lite crate
   - Store TOTP secrets encrypted
   - Generate 6-digit codes
   - QR code generation for setup (optional)

2. **Password Strength Scoring**
   - Integrate zxcvbn (or implement simplified version)
   - Score: 0-4 (very weak to very strong)
   - Show entropy calculation
   - Warn on weak passwords

3. **Breach Detection**
   - Implement k-anonymity password check
   - Local hash database support
   - SHA-1 hash first 5 chars lookup
   - Optional online API check (haveibeenpwned)

4. **Auto-Lock Mechanism**
   - Timeout after X minutes of inactivity
   - Lock vault automatically
   - Require re-authentication

5. **Encrypted Backup/Export**
   - Export vault to encrypted archive
   - Import from backup
   - Support multiple backup formats

**Estimated Work:** 5-7 features | 500-700 LOC

---

### Phase 3: User Experience Enhancements
**Priority: HIGH | Risk: LOW | Complexity: MEDIUM**

1. **Interactive CLI Mode**
   - Menu-driven interface using dialoguer or inquire
   - REPL-style interaction
   - Command history
   - Auto-completion

2. **Enhanced Output**
   - Better table formatting (prettytable-rs or comfy-table)
   - Progress indicators
   - More color schemes
   - ASCII art logo

3. **Confirmation Prompts**
   - Confirm destructive operations
   - Show what will be deleted
   - Undo/redo for last operation (optional)

4. **Favorites & Recent**
   - Mark entries as favorites
   - Track recently accessed entries
   - Quick access commands

**Estimated Work:** 4-5 features | 400-500 LOC

---

### Phase 4: Analytics & Insights
**Priority: MEDIUM | Risk: LOW | Complexity: MEDIUM**

1. **Password Analysis**
   - Detect weak passwords (score < 3)
   - Detect reused passwords
   - Detect old passwords (> 90 days)
   - Show duplicate usernames across services

2. **Vault Health Score**
   - Overall security score (0-100)
   - Breakdown by category
   - Recommendations

3. **Statistics Command**
   - Total credentials
   - By category
   - By age
   - By strength
   - Most/least used

**Estimated Work:** 3-4 features | 300-400 LOC

---

### Phase 5: Advanced Search & Organization
**Priority: MEDIUM | Risk: LOW | Complexity: MEDIUM**

1. **Fuzzy Search**
   - Use fuzzy-matcher or similar crate
   - Search across all fields
   - Ranked results

2. **Advanced Filtering & Sorting**
   - Sort by date added/modified/accessed
   - Sort by name
   - Filter by category/tag
   - Filter by age/strength

3. **Bulk Operations**
   - Rename services
   - Bulk tag/untag
   - Bulk export

**Estimated Work:** 3-4 features | 200-300 LOC

---

### Phase 6: Advanced Password Generation
**Priority: MEDIUM | Risk: LOW | Complexity: MEDIUM**

1. **Custom Character Sets**
   - Allow user-defined character sets
   - Exclude ambiguous characters (0/O, 1/l/I)
   - Include/exclude specific characters

2. **Diceware Passphrase Generator**
   - Use EFF word list
   - Generate memorable passphrases
   - Configurable word count
   - Custom separator

3. **Generator Presets**
   - Save favorite generator configurations
   - Named presets (e.g., "banking", "social", "email")
   - Default preset selection

4. **Username Generator**
   - Random username suggestions
   - Based on service patterns

**Estimated Work:** 4-5 features | 300-400 LOC

---

### Phase 7: Advanced Features
**Priority: LOW | Risk: MEDIUM | Complexity: HIGH**

1. **Audit Log**
   - Encrypted log of all operations
   - Timestamp + operation + user
   - View audit log command
   - Log rotation

2. **Read-Only Mode**
   - Open vault in read-only
   - Prevent modifications
   - Good for showing/copying only

3. **JSON Export (Encrypted)**
   - Export to structured JSON
   - Keep encryption
   - Import back

4. **Plugin Architecture**
   - Define plugin trait
   - Load plugins at runtime
   - Example plugins (password generator, breach checker)

**Estimated Work:** 4-5 features | 500-600 LOC

---

### Phase 8: Testing & Documentation
**Priority: HIGH | Risk: LOW | Complexity: MEDIUM**

1. **Unit Tests**
   - Test all new modules
   - Test edge cases
   - Test error conditions
   - Aim for 80%+ coverage

2. **Integration Tests**
   - End-to-end workflows
   - Multi-vault tests
   - Import/export tests

3. **Documentation**
   - Inline documentation comments
   - Update README with all new features
   - Create FEATURES.md
   - Update ARCHITECTURE.md

4. **Performance Tests**
   - Benchmark key operations
   - Test with large vaults (1000+ entries)

**Estimated Work:** Ongoing | 200-300 LOC

---

## Implementation Order (Recommended)

### Sprint 1: Foundation (Days 1-2)
- Enhanced Credential model with tags, favorites, last_accessed
- Vault metadata and settings
- Password strength scoring
- Weak password detection

### Sprint 2: Security (Days 3-4)
- TOTP support
- Auto-lock mechanism
- Encrypted backup/export
- Breach detection (basic)

### Sprint 3: UX (Days 5-6)
- Interactive CLI mode
- Enhanced output formatting
- Fuzzy search
- Favorites & recent entries

### Sprint 4: Analytics (Days 7-8)
- Vault health score
- Statistics command
- Reused password detection
- Old password warnings

### Sprint 5: Advanced (Days 9-10)
- Diceware generator
- Multi-vault support
- Audit log
- Advanced filters

### Sprint 6: Polish (Days 11-12)
- Comprehensive testing
- Documentation updates
- Performance optimization
- Bug fixes

---

## Technical Debt & Refactoring

1. **Modularization**
   - Split cli.rs into submodules (commands/, interactive/, output/)
   - Create analytics/ module
   - Create generators/ module
   - Create security/ module (breach, strength, totp)

2. **Error Handling**
   - More specific error types
   - Better error messages
   - Recovery suggestions

3. **Configuration**
   - Config file support (~/.config/password_manager/config.toml)
   - Per-vault settings
   - Global settings

4. **Code Quality**
   - Add clippy::pedantic
   - Add clippy::nursery
   - Increase test coverage
   - Add benchmarks

---

## Dependencies to Add

```toml
# Security
totp-lite = "2.0"           # TOTP generation
sha1 = "0.10"               # For breach checking

# Password strength
zxcvbn = "2.2"              # Password strength estimation

# CLI enhancements
dialoguer = "0.11"          # Interactive prompts
comfy-table = "7.1"         # Pretty tables
indicatif = "0.17"          # Progress bars
console = "0.15"            # Terminal utilities

# Search
fuzzy-matcher = "0.3"       # Fuzzy string matching

# Diceware
wordlist = "0.1"            # Or embed EFF wordlist

# Configuration
config = "0.14"             # Config file management
toml = "0.8"                # TOML parsing

# Utilities
humantime = "2.1"           # Human-readable durations
bytes = "1.5"               # Byte utilities
```

---

## Success Metrics

- [ ] 20+ new features implemented
- [ ] 2000+ lines of Rust code (total)
- [ ] 50+ unit tests
- [ ] 10+ integration tests
- [ ] 90%+ test coverage
- [ ] All clippy warnings resolved
- [ ] Comprehensive documentation
- [ ] No breaking changes to existing functionality

---

## Risk Mitigation

1. **Backward Compatibility**
   - Support vault version migration
   - Keep old format readable
   - Add migration tool if needed

2. **Security**
   - Audit all new crypto operations
   - No custom crypto implementations
   - Use well-tested crates only

3. **Performance**
   - Profile before optimizing
   - Keep O(n) operations for large vaults
   - Use indexes where appropriate

4. **Complexity**
   - Keep modules focused
   - Document complex logic
   - Refactor when modules exceed 500 LOC

---

## Post-Launch

- GitHub Actions CI/CD
- Release binaries for multiple platforms
- Homebrew formula
- AUR package
- Snap/Flatpak packages
- Security audit
- Performance benchmarks
- Community feedback incorporation
