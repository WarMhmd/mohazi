# Keyword: transform

Alias: -

Description: Applies a transformation function or operation to the field value before or after validation (e.g., trimming whitespace or converting to lowercase).

Accepted values: list of valid transformation function name (cast, join, split, etc.)

Example:
```yaml
profile:
    username:
        type: string
        transform: 
            toLowerCase: true
```
