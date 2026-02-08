# Keyword: error

Aliases: -

Accepted values: string

Applicable types: any (In the rules block)

Example:
```yaml
login:
    username:
        type: string
        rules:
            minLength: 10
            error: "Username must be at least 10 characters long."
```
