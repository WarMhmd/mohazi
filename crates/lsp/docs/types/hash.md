# Type Hash
The `hash` type validates that a given string matches the required structure of a selected cryptographic hash output pattern. Valid algorithms are: Argon2id, Bcrypt, SHA-256, SHA-512, and BLAKE3.

### Example
```yaml
Form1:
    hash_field:
        type: hash
        rules:
            algorithm: Argon2id
```