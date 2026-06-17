



export const validateHash_form = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.password_hash === undefined || data.password_hash === null) {
    result.errors.push({
        path: "password_hash",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.password_hash !== "string") {
    result.errors.push({
        path: "password_hash",
        message: "Invalid value."
    });
    flag = false;
} else {
    let isValidHash = false;
    isValidHash = /^\$argon2(id|i|d)\$v=\d+\$m=\d+,t=\d+,p=\d+\$[A-Za-z0-9+/]+={0,2}\$[A-Za-z0-9+/]+={0,2}$/.test(data.password_hash);

    if (!isValidHash) {
        result.errors.push({
            path: "password_hash",
            message: "Invalid value."
        });
        flag = false;
    }
    data.password_hash = data.password_hash.trim();
}if (flag) {
            successData.password_hash = data.password_hash;
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