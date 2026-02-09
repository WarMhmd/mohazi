# Rule: nonnegative

Aliases: -

Accepted values: boolean value (set to `true` to enforce non-negative numbers)

Applicable types: number

Example:
```yaml
inventory:
    stock:
        type: number
        rules:
            nonnegative: true
            error: "Stock count cannot be negative."
```
