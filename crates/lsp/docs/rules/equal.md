# Rule: equal

Aliases: -

Accepted values: numeric value (integer or float)

Applicable types: number

Example:
```yaml
system:
    status_code:
        type: number
        rules:
            equal: 200
            error: "System status must be exactly 200."
```
