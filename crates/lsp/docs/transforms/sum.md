# Transform: sum

Aliases: -

Description: Calculates the total sum of all numeric elements within an array.

Accepted values: boolean, set to `true` to enable

Applicable types: array

Example:
```yaml
invoice:
    total_amount:
        type: array
        rules:
            type: number
        transform:
            sum: true
```
