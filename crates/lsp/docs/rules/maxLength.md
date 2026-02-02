# Rule: maxLength

Aliases: -

Accepted values: integer representing the maximum allowed length

Applicable types: string, array

Example:
```yaml
post:
    title:
        type: string
        rules:
            maxLength: 100
            error: "Title must be less than 100 characters."
```
