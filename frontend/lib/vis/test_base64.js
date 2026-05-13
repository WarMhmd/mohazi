

import { isStrictBase64 } from "./utils/base64_validator.js";

export const validateAuth = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data["avatar"] === undefined || data["avatar"] === null) {
    result.errors.push({
        path: "avatar",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data["avatar"] !== "string") {
    result.errors.push({
        path: "avatar",
        message: "Invalid value."
    });
    flag = false;
} else if (!isStrictBase64(data["avatar"], false)) {
    result.errors.push({
        path: "avatar",
        message: "Invalid value."
    });
    flag = false;
} else {
    const decodedLength = typeof Buffer !== 'undefined' 
        ? Buffer.from(data["avatar"], 'base64').length 
        : atob(data["avatar"].replace(/-/g, '+').replace(/_/g, '/')).length;
    if (decodedLength > 1048576) {
        result.errors.push({
            path: "avatar",
            message: "Invalid value."
        });
        flag = false;
    }
}
if (flag) {
            successData.avatar = data.avatar;
        }
    }
    {
        flag = true;flag = true;

if (data["sessionToken"] === undefined || data["sessionToken"] === null) {
    result.errors.push({
        path: "sessionToken",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data["sessionToken"] !== "string") {
    result.errors.push({
        path: "sessionToken",
        message: "Invalid value."
    });
    flag = false;
} else if (!isStrictBase64(data["sessionToken"], true)) {
    result.errors.push({
        path: "sessionToken",
        message: "Invalid value."
    });
    flag = false;
} else {
    const decodedLength = typeof Buffer !== 'undefined' 
        ? Buffer.from(data["sessionToken"], 'base64url').length 
        : atob(data["sessionToken"].replace(/-/g, '+').replace(/_/g, '/')).length;
    if (decodedLength < 32) {
        result.errors.push({
            path: "sessionToken",
            message: "Invalid value."
        });
        flag = false;
    }
    if (decodedLength > 64) {
        result.errors.push({
            path: "sessionToken",
            message: "Invalid value."
        });
        flag = false;
    }
    data["sessionToken"] = data["sessionToken"].trim();
}
if (flag) {
            successData.sessionToken = data.sessionToken;
        }
    }
    {
        flag = true;flag = true;

if (data.tags === undefined || data.tags === null) {
    result.errors.push({
        path: "tags",
        message: "Invalid value."
    });
    flag = false;
} else if (!Array.isArray(data.tags)) {
    result.errors.push({
        path: "tags",
        message: "Invalid value."
    });
    flag = false;
} else {

    for (let i = 0; i < data.tags.length; i++) {
        const previousFlag = flag;
        flag = true;

        const itemPath = "tags" + "[" + i + "]";
        data[itemPath] = data.tags[i];flag = true;

if (data[itemPath] === undefined || data[itemPath] === null) {
    result.errors.push({
        path: itemPath,
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data[itemPath] !== "string") {
    result.errors.push({
        path: itemPath,
        message: "Invalid value."
    });
    flag = false;
} else if (!isStrictBase64(data[itemPath], false)) {
    result.errors.push({
        path: itemPath,
        message: "Invalid value."
    });
    flag = false;
} else {
}


        data.tags[i] = data[itemPath];
        delete data[itemPath];

        const itemFlag = flag;
        flag = previousFlag && itemFlag;}
}if (flag) {
            successData.tags = data.tags;
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