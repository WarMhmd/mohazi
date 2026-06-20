# Rule algorithm
The `algorithm` rule specifies the hashing algorithm validation expected for a `hash` field.

Supported values include:
- `Argon2id`
- `Bcrypt`
- `SHA-256`
- `SHA-512`
- `BLAKE3`

### Example
```yaml
Form1:
    password_hash:
        type: hash
        rules:
            algorithm: Bcrypt
```