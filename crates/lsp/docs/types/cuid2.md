# CUID2 Field Type

The `cuid2` field type represents a cryptographically secure, collision-resistant identifier. It is the successor to CUID and should always be preferred for large-scale distributed systems.

## Characteristics
- **Collision Resistance:** Designed for massive scale.
- **Security:** Cryptographically secure.
- **Portability:** URL-friendly and case-insensitive (lowercase).
- **Inheritance:** Inherits from `String`.

## Rules
It supports all `String` rules, with the following defaults:
- `minLength`: 9
- `maxLength`: 31
- `regex`: `^[a-z][a-z0-9]*$`

## Example
```yaml
user:
  id:
    type: cuid2
    rules:
      minLength: 10
      maxLength: 32
```

## Warning
**Do not use CUID (v1).** It was deprecated because it is not cryptographically secure and has collision risks. **Always use CUID2.**
