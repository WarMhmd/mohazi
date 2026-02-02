# Rule: type

Aliases: -

Accepted values: data type (string, number, boolean, etc.)

Applicable types: array

Example:
```yaml
gallery:
    images:
        type: array
        rules:
            type: string
            error: "All image paths must be strings."
scores:
    points:
        type: array
        rules:
            type: number
            error: "All points in the list must be numeric."
```
