



export const validateMail_form = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.user_email === undefined || data.user_email === null) {
    result.errors.push({
        path: "user_email",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.user_email !== "string") {
    result.errors.push({
        path: "user_email",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (!new RegExp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9]?(?:\\.[a-zA-Z0-9]?)*$").test(data.user_email)) {
        result.errors.push({
            path: "user_email",
            message: "Invalid value."
        });
        flag = false;
    }

    if (flag) {
        const domain = data.user_email.split('@').pop();
        const allowedDomains = ["gmail.com","outlook.com"];
        if (!allowedDomains.includes(domain)) {
            result.errors.push({
                path: "user_email",
                message: "Email domain not allowed"
            });
            flag = false;
        }
    }
    data.user_email = data.user_email.trim();
}if (flag) {
            successData.user_email = data.user_email;
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