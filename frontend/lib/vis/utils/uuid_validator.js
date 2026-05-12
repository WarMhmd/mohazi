/**
 * Bundled UUID Validator (NPM Equivalent)
 */

const UUID_REGEX = /^(?:[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}|00000000-0000-0000-0000-000000000000)$/i;

export const validateUuid = (uuid) => {
    return typeof uuid === 'string' && UUID_REGEX.test(uuid);
};

export const getUuidVersion = (uuid) => {
    if (!validateUuid(uuid)) return null;
    return parseInt(uuid.charAt(14), 16);
};
