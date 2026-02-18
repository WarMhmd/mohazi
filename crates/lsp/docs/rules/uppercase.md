# Rule: uppercase

Aliases: -

Accepted values: boolean value (set to `true` to enforce uppercase)

Applicable types: string

Example:
```yaml
identity:
    state_code:
        type: string
        rules:
            uppercase: true
            error: "State code must be in uppercase."
```
