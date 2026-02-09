# Keyword: rules

Alias: -

Description: A container for a list of validation logic or sub-rules that must be applied to the field.

Accepted values: list of rules or an object

Example:
```yaml
inventory:
    stock_count:
        type: number
        rules:
            min: 0
            error: "Stock count must be greater than 0."
            max: 
                value: 100
                error: "Stock count must be less than 100."
            nonnegative: {"value": true, "error": "Stock count must be non-negative."}
```
