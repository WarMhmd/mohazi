# Transform: trim

Aliases: -

Description: Removes whitespace from both the beginning and the end of a string.

Accepted values: boolean, set to `true` to enable

Applicable types: string

Example:
```yaml
profile:
    username:
        type: string
        transform:
            trim: true
```
