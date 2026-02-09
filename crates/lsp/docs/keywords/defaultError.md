# Keyword: defaultError

Alias: -

Description: Defines a custom error message to be returned by the compiler if the rule error is not explicitly defined.

Accepted values: string

Example:
```yaml
registration:
    email:
        type: string
        required: true
        defaultError: "A valid email address is required to create an account."
```
