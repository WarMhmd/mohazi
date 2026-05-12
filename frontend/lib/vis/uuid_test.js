
import { validateUuid, getUuidVersion } from "./utils/uuid_validator.js";

export const validateUuidtest = async (data) => {
	let result = {
		success: false,
		errors: [],
		data: data
	};

	const successData = {};
	let flag = true;
	{
		flag = true;
		if (data.legacyId === undefined || data.legacyId === null) {
	result.errors.push({
		path: "legacyId",
		message: "Invalid value."
	});
	flag = false;
} else if (typeof data.legacyId !== "string") {
	result.errors.push({
		path: "legacyId",
		message: "Invalid value."
	});
	flag = false;
} else {

	if (flag) {
		let uuidFlag = true;
		if (!validateUuid(data.legacyId)) {
			uuidFlag = false;
		}
		else if (getUuidVersion(data.legacyId) !== 1) {
			uuidFlag = false;
		}

		if (!uuidFlag) {
			result.errors.push({
				path: "legacyId",
				message: "Invalid UUID version"
			});
			flag = false;
		}
	}
	if (flag) {
		successData.legacyId = data.legacyId;
	}
}

	}
	{
		flag = true;
		if (data.randomId === undefined || data.randomId === null) {
	result.errors.push({
		path: "randomId",
		message: "Invalid value."
	});
	flag = false;
} else if (typeof data.randomId !== "string") {
	result.errors.push({
		path: "randomId",
		message: "Invalid value."
	});
	flag = false;
} else {

	if (flag) {
		let uuidFlag = true;
		if (!validateUuid(data.randomId)) {
			uuidFlag = false;
		}
		else if (getUuidVersion(data.randomId) !== 4) {
			uuidFlag = false;
		}

		if (!uuidFlag) {
			result.errors.push({
				path: "randomId",
				message: "Invalid UUID version"
			});
			flag = false;
		}
	}
	if (flag) {
		successData.randomId = data.randomId;
	}
}

	}
	{
		flag = true;
		if (data.sortableId === undefined || data.sortableId === null) {
	result.errors.push({
		path: "sortableId",
		message: "Invalid value."
	});
	flag = false;
} else if (typeof data.sortableId !== "string") {
	result.errors.push({
		path: "sortableId",
		message: "Invalid value."
	});
	flag = false;
} else {

	if (flag) {
		let uuidFlag = true;
		if (!validateUuid(data.sortableId)) {
			uuidFlag = false;
		}
		else if (getUuidVersion(data.sortableId) !== 7) {
			uuidFlag = false;
		}

		if (!uuidFlag) {
			result.errors.push({
				path: "sortableId",
				message: "Invalid UUID version"
			});
			flag = false;
		}
	}
	if (flag) {
		successData.sortableId = data.sortableId;
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