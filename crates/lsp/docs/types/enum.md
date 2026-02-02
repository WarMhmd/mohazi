# Type: enum

Description: Restricts the field value to a predefined set of allowed values.

Example:
```yaml
order:
    status:
        type: enum
        values: ["pending", "shipped", "delivered", "cancelled"]
```
