# Rule: length

Aliases: -

Accepted values: integer representing the exact required length

Applicable types: string, array

Example:
```yaml
auth:
    pincode:
        type: string
        rules:
            length: 4
            error: "Pincode must be 4 digits."
```
