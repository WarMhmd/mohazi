using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Generated.Validation.Utils;

namespace Generated.Validation
{
	public static class Validators
	{
		public static async Task<ValidationResult> ValidateUuidtest(Dictionary<string, object?> data)
		{
			var result = new ValidationResult
			{
				Success = false,
				Errors = new List<ValidationError>(),
				Data = data
			};

			var successData = new Dictionary<string, object?>();
			bool flag = true;
			{
				flag = true;
				if (!data.TryGetValue("legacyId", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "legacyId",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "legacyId",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["legacyId"] = value;

	if (flag)
	{
		bool uuidFlag = true;
		if (!UuidValidator.Validate(value))
		{
			uuidFlag = false;
		}
		else if (UuidValidator.GetVersion(value) != 1)
		{
			uuidFlag = false;
		}

		if (!uuidFlag)
		{
			result.Errors.Add(new ValidationError
			{
				Path = "legacyId",
				Message = "Invalid UUID version"
			});
			flag = false;
		}
	}
	if (flag)
	{
		successData["legacyId"] = data["legacyId"];
	}
}

			}
			{
				flag = true;
				if (!data.TryGetValue("randomId", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "randomId",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "randomId",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["randomId"] = value;

	if (flag)
	{
		bool uuidFlag = true;
		if (!UuidValidator.Validate(value))
		{
			uuidFlag = false;
		}
		else if (UuidValidator.GetVersion(value) != 4)
		{
			uuidFlag = false;
		}

		if (!uuidFlag)
		{
			result.Errors.Add(new ValidationError
			{
				Path = "randomId",
				Message = "Invalid UUID version"
			});
			flag = false;
		}
	}
	if (flag)
	{
		successData["randomId"] = data["randomId"];
	}
}

			}
			{
				flag = true;
				if (!data.TryGetValue("sortableId", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "sortableId",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "sortableId",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["sortableId"] = value;

	if (flag)
	{
		bool uuidFlag = true;
		if (!UuidValidator.Validate(value))
		{
			uuidFlag = false;
		}
		else if (UuidValidator.GetVersion(value) != 7)
		{
			uuidFlag = false;
		}

		if (!uuidFlag)
		{
			result.Errors.Add(new ValidationError
			{
				Path = "sortableId",
				Message = "Invalid UUID version"
			});
			flag = false;
		}
	}
	if (flag)
	{
		successData["sortableId"] = data["sortableId"];
	}
}

			}

			if (result.Errors.Count == 0)
			{
				result = new ValidationResult
				{
					Success = true,
					Errors = new List<ValidationError>(),
					Data = successData
				};
			}

			return result;
		}
	}
}