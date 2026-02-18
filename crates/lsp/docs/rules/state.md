# Rule: state

Aliases: value

Accepted values: boolean (`true` or `false`)

Applicable types: boolean

Example:
```yaml
terms:
    agreement:
        type: boolean
        rules:
            state: true
            error: "You must agree to the terms to continue."
system:
    maintenance_mode:
        type: boolean
        rules:
            state: false
            error: "Action cannot be performed while maintenance is active."
```
