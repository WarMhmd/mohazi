# Username Type

A specialized string type for user handles.

## Rules

Inherits all [String Rules](./string.md).

### Default Regex
`^[a-zA-Z0-9](?:[._-]?[a-zA-Z0-9]){2,29}$`

Matches common username patterns (GitHub, Twitter/X, Instagram).

## Transforms

Inherits all [String Transforms](../transforms/cast.md).
Default: cast to `string`.
