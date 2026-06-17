



export const validateUsername_form = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.profile_name === undefined || data.profile_name === null) {
    result.errors.push({
        path: "profile_name",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.profile_name !== "string") {
    result.errors.push({
        path: "profile_name",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (data.profile_name.length < 5) {
        result.errors.push({
            path: "profile_name",
            message: "Invalid value."
        });
        flag = false;
    }
    if (!new RegExp("^[a-zA-Z0-9](?:[._-]?[a-zA-Z0-9]){2,29}$").test(data.profile_name)) {
        result.errors.push({
            path: "profile_name",
            message: "Invalid value."
        });
        flag = false;
    }
    data.profile_name = data.profile_name.trim();
}
if (flag) {
            successData.profile_name = data.profile_name;
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