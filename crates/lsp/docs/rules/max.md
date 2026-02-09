# Rule: max

Aliases: lte, lessThanOrEqual

Accepted values: numeric value (integer or float)

Applicable types: number

Example:
```yaml
survey:
    rating:
        type: number
        rules:
            max: 5
            error: "Rating cannot exceed 5."
```
