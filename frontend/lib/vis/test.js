import { validateFileSignature } from "./utils/file_signature.js";


export const validateLogin = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.password === undefined || data.password === null) {
    result.errors.push({
        path: "password",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.password !== "string") {
    result.errors.push({
        path: "password",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (data.password.length < 8) {
        result.errors.push({
            path: "password",
            message: Password must be at least 8 characters long.
        });
        flag = false;
    }
    if (data.password.length > 100) {
        result.errors.push({
            path: "password",
            message: Password must be at most 100 characters long.
        });
        flag = false;
    }

    if (flag) {
        successData.password = data.password;
    }
}}
    {
        flag = true;if (data.profile === undefined || data.profile === null) {
	result.errors.push({
		path: "profile",
		message: "\"Invalid value.\""
	});
	flag = false;
} else if (typeof data.profile !== "string") {
	result.errors.push({
		path: "profile",
		message: "\"Invalid value.\""
	});
	flag = false;
} else {
	if (!await validateFileExtension(data.profile, ["jpg","jpeg","png","webp","avif","heic","heif"])) {
		result.errors.push({
			path: "profile",
			message: "\"Invalid value.\""
		});
		flag = false;
	}

	if (flag) {
		const dimensions = await getImageDimensions(data.profile);
		if (dimensions.width !== 512) {
			result.errors.push({
				path: "profile",
				message: "Invalid image width"
			});
			flag = false;
		}
		if (dimensions.height !== 512) {
			result.errors.push({
				path: "profile",
				message: "Invalid image height"
			});
			flag = false;
		}
	}

	if (flag) {
		successData.profile = data.profile;
	}
}
}
    {
        flag = true;flag = true;

if (data.username === undefined || data.username === null) {
    result.errors.push({
        path: "username",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.username !== "string") {
    result.errors.push({
        path: "username",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (data.username.length < 3) {
        result.errors.push({
            path: "username",
            message: Username must be at least 3 characters long.
        });
        flag = false;
    }
    if (data.username.length > 20) {
        result.errors.push({
            path: "username",
            message: Username must be at most 20 characters long.
        });
        flag = false;
    }

    if (flag) {
        successData.username = data.username;
    }
}}

    if (result.errors.length === 0) {
        result = {
            success: true,
            errors: [],
            data: successData
        };
    }

    return result;
};

export const validateRegister = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true;flag = true;

if (data.email === undefined || data.email === null) {
    result.errors.push({
        path: "email",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.email !== "string") {
    result.errors.push({
        path: "email",
        message: "Invalid value."
    });
    flag = false;
} else {

    if (flag) {
        successData.email = data.email;
    }
}}
    {
        flag = true;flag = true;

if (data.password === undefined || data.password === null) {
    result.errors.push({
        path: "password",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.password !== "string") {
    result.errors.push({
        path: "password",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (data.password.length < 8) {
        result.errors.push({
            path: "password",
            message: Password must be at least 8 characters long.
        });
        flag = false;
    }
    if (data.password.length > 100) {
        result.errors.push({
            path: "password",
            message: Password must be at most 100 characters long.
        });
        flag = false;
    }

    if (flag) {
        successData.password = data.password;
    }
}}
    {
        flag = true;flag = true;

if (data.username === undefined || data.username === null) {
    result.errors.push({
        path: "username",
        message: "Invalid value."
    });
    flag = false;
} else if (typeof data.username !== "string") {
    result.errors.push({
        path: "username",
        message: "Invalid value."
    });
    flag = false;
} else {
    if (data.username.length < 3) {
        result.errors.push({
            path: "username",
            message: Username must be at least 3 characters long.
        });
        flag = false;
    }
    if (data.username.length > 20) {
        result.errors.push({
            path: "username",
            message: Username must be at most 20 characters long.
        });
        flag = false;
    }

    if (flag) {
        successData.username = data.username;
    }
}}

    if (result.errors.length === 0) {
        result = {
            success: true,
            errors: [],
            data: successData
        };
    }

    return result;
};