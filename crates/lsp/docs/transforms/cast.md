# Transform: cast

Aliases: -

Description: Forcibly converts the input value to the target data type (e.g., converting the string "123" to the number 123).

Accepted values: data type (string, number, boolean, etc.)

Applicable types: string, number, boolean, file, enum, array

Example:
```yaml
config:
    port:
        type: number
        transform:
            cast: string
```
