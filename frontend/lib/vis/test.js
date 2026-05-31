

export const validateLogin = async (data) => {
	let result = {
		success: false,
		errors: [],
		data: data
	};

	const successData = {};
	let flag = true;
	{
		flag = true;
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
			message: "Password must be at least 8 characters long."
		});
		flag = false;
	}
	if (data.password.length > 100) {
		result.errors.push({
			path: "password",
			message: "Password must be at most 100 characters long."
		});
		flag = false;
	}
	if (!new RegExp("^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$").test(data.password)) {
		result.errors.push({
			path: "password",
			message: "Password must contain at least one letter and one number."
		});
		flag = false;
	}
	if (flag) {
		successData.password = data.password;
	}
}

	}
	{
		flag = true;
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
			message: "Username must be at least 3 characters long."
		});
		flag = false;
	}
	if (data.username.length > 20) {
		result.errors.push({
			path: "username",
			message: "Username must be at most 20 characters long."
		});
		flag = false;
	}
	if (!new RegExp("^[a-zA-Z0-9_]+$").test(data.username)) {
		result.errors.push({
			path: "username",
			message: "Username can only contain letters, numbers, and underscores."
		});
		flag = false;
	}
	if (flag) {
		successData.username = data.username;
	}
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

export const validateRegister = async (data) => {
	let result = {
		success: false,
		errors: [],
		data: data
	};

	const successData = {};
	let flag = true;
	{
		flag = true;
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
	if (!new RegExp("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$").test(data.email)) {
		result.errors.push({
			path: "email",
			message: "Please enter a valid email address."
		});
		flag = false;
	}
	if (flag) {
		successData.email = data.email;
	}
}

	}
	{
		flag = true;
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
			message: "Password must be at least 8 characters long."
		});
		flag = false;
	}
	if (data.password.length > 100) {
		result.errors.push({
			path: "password",
			message: "Password must be at most 100 characters long."
		});
		flag = false;
	}
	if (!new RegExp("^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$").test(data.password)) {
		result.errors.push({
			path: "password",
			message: "Password must contain at least one letter and one number."
		});
		flag = false;
	}
	if (flag) {
		successData.password = data.password;
	}
}

	}
	{
		flag = true;
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
			message: "Username must be at least 3 characters long."
		});
		flag = false;
	}
	if (data.username.length > 20) {
		result.errors.push({
			path: "username",
			message: "Username must be at most 20 characters long."
		});
		flag = false;
	}
	if (!new RegExp("^[a-zA-Z0-9_]+$").test(data.username)) {
		result.errors.push({
			path: "username",
			message: "Username can only contain letters, numbers, and underscores."
		});
		flag = false;
	}
	if (flag) {
		successData.username = data.username;
	}
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