
import { validateUuid, getUuidVersion } from "./utils/uuid_validator.js";


export const validateUuid_form = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.session_id === undefined || data.session_id === null) {
    result.errors.push({
        path: "session_id",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.session_id !== "string") {
    result.errors.push({
        path: "session_id",
        message: "Invalid value."
    });
    flag = false;
} else {
    let uuidFlag = true;
    if (!validateUuid(data.session_id)) {
        uuidFlag = false;
    }
    else if (getUuidVersion(data.session_id) !== 4) {
        uuidFlag = false;
    }

    if (!uuidFlag) {
        result.errors.push({
            path: "session_id",
            message: "Invalid UUID version"
        });
        flag = false;
    }
    data.session_id = data.session_id.trim();
}
if (flag) {
            successData.session_id = data.session_id;
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