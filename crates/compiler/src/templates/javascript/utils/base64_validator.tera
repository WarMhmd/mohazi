/**
 * Bundled Base64 Validator
 */

const BASE64_REGEX = /^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/;
const BASE64_URL_REGEX = /^(?:[A-Za-z0-9_-]{4})*(?:[A-Za-z0-9_-]{2}==|[A-Za-z0-9_-]{3}=)?$/;

export function isStrictBase64(str, url = false) {
    if (typeof str !== 'string') return false;
    
    // 1. Basic length check (Base64 must be multiple of 4)
    if (str.length % 4 !== 0) return false;

    // 2. Regex check for invalid characters
    const regex = url ? BASE64_URL_REGEX : BASE64_REGEX;
    if (!regex.test(str)) return false;

    // 3. The "Round Trip" Integrity Check
    try {
        if (typeof Buffer !== 'undefined') {
            // Node.js environment
            const encoding = url ? 'base64url' : 'base64';
            // base64url support was added in Node.js v14.18.0, v16.0.0
            // For older versions or better compatibility, we can manually convert
            if (url) {
                const normal = str.replace(/-/g, '+').replace(/_/g, '/');
                const buf = Buffer.from(normal, 'base64');
                const roundTrip = buf.toString('base64').replace(/\+/g, '-').replace(/\//g, '_');
                return roundTrip === str;
            } else {
                return Buffer.from(str, 'base64').toString('base64') === str;
            }
        } else {
            // Browser environment
            if (url) {
                const normal = str.replace(/-/g, '+').replace(/_/g, '/');
                const decoded = atob(normal);
                const roundTrip = btoa(decoded).replace(/\+/g, '-').replace(/\//g, '_');
                return roundTrip === str;
            } else {
                return btoa(atob(str)) === str;
            }
        }
    } catch (err) {
        return false;
    }
}
