# Rule: gt

Aliases: greaterThan

Accepted values: numeric value (integer or float)

Applicable types: number

Example:
```yaml
game:
    player_count:
        type: number
        rules:
            gt: 0
            error: "At least one player is required."
```
