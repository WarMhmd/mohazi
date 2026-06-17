# Mail Type
The `mail` type is used for validating email addresses. It inherits all properties from the `string` type but provides specialized rules for domain validation and a default RFC 5322 compliant regex.

## Default Behavior
- Default `cast`: `string`
- Default `regex`: RFC 5322 compliant pattern.

## Example
```yaml
email:
  type: mail
  rules:
    allowedDomains: ["google.com", "outlook.com"]
```
