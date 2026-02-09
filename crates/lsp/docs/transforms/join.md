# Transform: join

Aliases: -

Description: Combines all elements of an array into a single string, separated by a specified delimiter.

Accepted values: string (the separator character or sequence)

Applicable types: array

Example:
```yaml
output:
    tags_string:
        type: string
        transform:
            join: "; "
```
