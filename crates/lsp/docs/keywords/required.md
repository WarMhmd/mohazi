# Keyword: required

Alias: -

Description: tell the compiler whether this field is required or not.

Accepted values: `true` or `false`

Example:
```yaml
login:
    username:
        type: string
        required: true
    age:
        type: number
        required: false
```
