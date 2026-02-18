# Rule: values

Aliases: -

Accepted values: string | string array to define values of the enum

Applicable types: enum

Example:
```yaml
order:
    status:
        type: enum
        rules:
            values: ["pending", "shipped", "delivered", "cancelled"]
```
