# Rule: multipleOf

Aliases: -

Accepted values: numeric value (integer or float)

Applicable types: number

Example:
```yaml
shipping:
    box_size:
        type: number
        rules:
            multipleOf: 12
            error: "Box size must be a multiple of 12."
```
