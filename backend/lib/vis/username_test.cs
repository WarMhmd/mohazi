using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Generated.Validation
{
	public static class Validators
	{
		public static async Task<ValidationResult> ValidateAuth(Dictionary<string, object?> data)
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
				if (!data.TryGetValue("handle", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "handle",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "handle",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["handle"] = value;
	if (value.Length < 3)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "handle",
			Message = "Invalid value."
		});
		flag = false;
	}
	if (value.Length > 15)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "handle",
			Message = "Invalid value."
		});
		flag = false;
	}
	if (flag)
	{
		successData["handle"] = data["handle"];
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