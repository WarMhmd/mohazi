# Rule: nonpositive

Aliases: -

Accepted values: boolean value (set to `true` to enforce non-positive numbers)

Applicable types: number

Example:
```yaml
physics:
    temperature_offset:
        type: number
        rules:
            nonpositive: true
            error: "Offset must be zero or less."
```
