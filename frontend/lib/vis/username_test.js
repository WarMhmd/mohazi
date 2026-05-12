

export const validateAuth = async (data) => {
	let result = {
		success: false,
		errors: [],
		data: data
	};

	const successData = {};
	let flag = true;
	{
		flag = true;
		if (data.handle === undefined || data.handle === null) {
	result.errors.push({
		path: "handle",
		message: "Invalid value."
	});
	flag = false;
} else if (typeof data.handle !== "string") {
	result.errors.push({
		path: "handle",
		message: "Invalid value."
	});
	flag = false;
} else {
	if (data.handle.length < 3) {
		result.errors.push({
			path: "handle",
			message: "Invalid value."
		});
		flag = false;
	}
	if (data.handle.length > 15) {
		result.errors.push({
			path: "handle",
			message: "Invalid value."
		});
		flag = false;
	}
	if (flag) {
		successData.handle = data.handle;
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