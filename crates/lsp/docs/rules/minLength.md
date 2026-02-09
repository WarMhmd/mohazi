# Rule: minLength

Aliases: -

Accepted values: integer representing the minimum allowed length

Applicable types: string, array

Example:
```yaml
profile:
    bio:
        type: string
        rules:
            minLength: 20
            error: "Bio must be at least 20 characters."
```
