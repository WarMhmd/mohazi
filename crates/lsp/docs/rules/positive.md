# Rule: positive

Aliases: -

Accepted values: boolean value (set to `true` to enforce positive numbers)

Applicable types: number

Example:
```yaml
wallet:
    balance:
        type: number
        rules:
            positive: true
            error: "Balance must be a positive number."
```
