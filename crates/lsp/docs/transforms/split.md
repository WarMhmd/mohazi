# Transform: split

Aliases: -

Description: Breaks a string into an array based on a specified delimiter.

Accepted values: string (the delimiter character or sequence)

Applicable types: string

Example:
```yaml
search:
    tags:
        type: array
        transform:
            split: ","
```
