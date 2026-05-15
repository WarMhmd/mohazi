

import { isStrictBase64 } from "./utils/base64_validator.js";

export const validateBase64_form = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.standard === undefined || data.standard === null) {
    result.errors.push({
        path: "standard",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.standard !== "string") {
    result.errors.push({
        path: "standard",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (!isStrictBase64(data.standard, false)) {
        result.errors.push({
            path: "standard",
            message: "Invalid value."
        });
        flag = false;
    }

    if (flag) {
        if (flag) {
            const decodedLength = typeof Buffer !== 'undefined' 
                ? Buffer.from(data.standard, 'base64').length 
                : atob(data.standard.replace(/-/g, '+').replace(/_/g, '/')).length;
            if (decodedLength < 10) {
                result.errors.push({
                    path: "standard",
                    message: "Base64 too small"
                });
                flag = false;
            }
        }
    }
}
if (flag) {
            successData.standard = data.standard;
        }
    }
    {
        flag = true;flag = true;

if (data.url_safe === undefined || data.url_safe === null) {
    result.errors.push({
        path: "url_safe",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.url_safe !== "string") {
    result.errors.push({
        path: "url_safe",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (!isStrictBase64(data.url_safe, true)) {
        result.errors.push({
            path: "url_safe",
            message: "Invalid value."
        });
        flag = false;
    }

    if (flag) {
        const hasPadding = data.url_safe.endsWith('=');
        if (hasPadding !== false) {
            result.errors.push({
                path: "url_safe",
                message: "Invalid base64 padding"
            });
            flag = false;
        }
    }
    data.url_safe = data.url_safe.trim();
}
if (flag) {
            successData.url_safe = data.url_safe;
        }
    }

    if (result.errors.length === 0) {
        result = {
            success: true,
            errors: [],
            data: successData
        };
    }

    return result;
};