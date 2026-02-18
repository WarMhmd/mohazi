# Rule: startsWith

Aliases: -

Accepted values: the string that the field must begin with

Applicable types: string

Example:
```yaml
finance:
    iban:
        type: string
        rules:
            startsWith: "JO"
            error: "IBAN must start with 'JO'."
```
