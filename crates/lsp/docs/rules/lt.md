# Rule: lt

Aliases: lessThan

Accepted values: numeric value (integer or float)

Applicable types: number

Example:
```yaml
entry:
    discount_percentage:
        type: number
        rules:
            lt: 100
            error: "Discount must be less than 100%."
```
