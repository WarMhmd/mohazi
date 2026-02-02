# Rule: regex

Aliases: pattern

Accepted values: a valid regular expression string

Applicable types: string

Example:
```yaml
account:
    username:
        type: string
        rules:
            regex: "^[a-zA-Z0-9_]+$"
            error: "Username must only contain letters, numbers, and underscores."
```
