



export const validateCuid2_form = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.id === undefined || data.id === null) {
    result.errors.push({
        path: "id",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.id !== "string") {
    result.errors.push({
        path: "id",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (data.id.length < 10) {
        result.errors.push({
            path: "id",
            message: "Invalid value."
        });
        flag = false;
    }
    if (data.id.length > 31) {
        result.errors.push({
            path: "id",
            message: "Invalid value."
        });
        flag = false;
    }
    if (!new RegExp("^c[a-z0-9]+$").test(data.id)) {
        result.errors.push({
            path: "id",
            message: "Invalid value."
        });
        flag = false;
    }
    data.id = data.id.trim();
}
if (flag) {
            successData.id = data.id;
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