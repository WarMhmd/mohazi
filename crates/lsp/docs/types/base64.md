# Base64 Type

The `base64` type validates that a string is a valid Base64 encoded string. It supports both standard Base64 and URL-safe Base64.

## Rules

### `url`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: If set to `true`, the validator will check for URL-safe Base64 encoding (using `-` and `_` instead of `+` and `/`).

### `minSize`
- **Type**: `number`
- **Description**: The minimum size of the **decoded** data in bytes.

### `maxSize`
- **Type**: `number`
- **Description**: The maximum size of the **decoded** data in bytes.

## Inherited String Rules

The `base64` type inherits all rules from the `string` type:
- `minLength`: Minimum length of the Base64 string.
- `maxLength`: Maximum length of the Base64 string.
- `length`: Exact length of the Base64 string.
- `pattern` / `regex`: Regular expression to match against the Base64 string.
- `startsWith`: String that the Base64 string must start with.
- `endsWith`: String that the Base64 string must end with.
- `includes`: String that the Base64 string must include.
- `uppercase`: If `true`, the Base64 string must be all uppercase.
- `lowercase`: If `true`, the Base64 string must be all lowercase.

## Transforms

The `base64` type supports standard string transforms:
- `trim`: Trims whitespace from both ends.
- `toLowerCase`: Converts to lowercase.
- `toUpperCase`: Converts to uppercase.
- `split`: Splits the string into an array.
