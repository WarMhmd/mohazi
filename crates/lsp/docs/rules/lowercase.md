# Rule: lowercase

Aliases: -

Accepted values: boolean value (set to `true` to enforce lowercase)

Applicable types: string

Example:
```yaml
user:
    slug:
        type: string
        rules:
            lowercase: true
            error: "Slug must be in lowercase."
```
