# Rule: negative

Aliases: -

Accepted values: boolean value (set to `true` to enforce negative numbers)

Applicable types: number

Example:
```yaml
finance:
    debt:
        type: number
        rules:
            negative: true
            error: "Value must be a negative number."
```
