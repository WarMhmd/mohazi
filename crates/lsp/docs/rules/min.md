# Rule: min

Aliases: gte, greaterThanOrEqual

Accepted values: numeric value (integer or float)

Applicable types: number

Example:
```yaml
product:
    price:
        type: number
        rules:
            min: 10.5
            error: "Price must be at least 10.5."
```
