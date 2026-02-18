# Rule: endsWith

Aliases: -

Accepted values: the string that the field must end with

Applicable types: string

Example:
```yaml
files:
    report:
        type: string
        rules:
            endsWith: ".pdf"
            error: "Report must end with '.pdf'."
```
