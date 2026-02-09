# Rule: includes

Aliases: -

Accepted values: a substring that must exist within the field

Applicable types: string

Example:
```yaml
search:
    query:
        type: string
        rules:
            includes: "verify"
            error: "Query must include 'verify'."
```
