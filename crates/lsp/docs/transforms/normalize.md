# Transform: normalize

Aliases: -

Description: Normalizes the string to a specific Unicode normalization form.

Accepted values: string (NFC, NFD, NFKC, NFKD)

Applicable types: string

Example:
```yaml
content:
    title:
        type: string
        transform:
            normalize: "NFC"
```
