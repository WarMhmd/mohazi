# Transform: toUpperCase

Aliases: uppercase

Description: Converts all characters in a string to uppercase.

Accepted values: boolean, set to `true` to enable

Applicable types: string

Example:
```yaml
finance:
    currency_code:
        type: string
        transform:
            toUpperCase: true
```
