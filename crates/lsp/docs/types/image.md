# Image Type

The `image` type is used for validating image files. It inherits all rules from the `file` type and adds specific validations for image dimensions and aspect ratio.

## Rules

- `width`: Exact width in pixels.
- `height`: Exact height in pixels.
- `minWidth`: Minimum width in pixels.
- `maxWidth`: Maximum width in pixels.
- `minHeight`: Minimum height in pixels.
- `maxHeight`: Maximum height in pixels.
- `ratio`: Aspect ratio (e.g., "1:1", "16:9").
- `extension` / `mime`: Allowed file extensions or MIME types. Defaults to common image formats.
- `maxSize`: Maximum file size.
- `minSize`: Minimum file size.

## Example

```yaml
profile_picture:
  type: image
  rules:
    maxWidth: 1024
    maxHeight: 1024
    ratio: 1:1
```
