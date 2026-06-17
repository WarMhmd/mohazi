# UUID Type

A specialized string type for Universally Unique Identifiers (UUIDs).

## Rules

Inherits all [String Rules](./string.md).

### version
`enum: ["v4", "v7"]` (Recommended)

Allows specifying the UUID version to validate against.

| **Version** | **Type** | **Secure?** | **Use Case** |
| --- | --- | --- | --- |
| **v1** | Time + MAC Address | ❌ **DEPRECATED** | Includes hardware MAC address. |
| **v2** | DCE Security | ❌ **DEPRECATED** | Obsolete. |
| **v3** | Name (MD5) | ❌ **DEPRECATED** | MD5 is broken. |
| **v4** | **Random** | ✅ **SECURE** | Best for secrets and temporary tokens. |
| **v5** | Name (SHA-1) | ⚠️ **DEPRECATED** | Use for deterministic IDs if needed. |
| **v6** | Reordered Time | ⚠️ **DEPRECATED** | v7 is preferred. |
| **v7** | **Time + Random** | ✅ **SECURE** | Best for Databases (Time-sortable). |
| **v8** | Custom | ❓ **DEPRECATED** | Experimental. |

## Transforms

Inherits all [String Transforms](../transforms/cast.md).
Default: cast to `string`.
