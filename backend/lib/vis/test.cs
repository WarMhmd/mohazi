using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Generated.Validation.Utils;
using Utils.FileSignatureValidator;

namespace Generated.Validation
{
	public static class Validators
	{
		public static async Task<ValidationResult> ValidateLogin(Dictionary<string, object?> data)
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
				if (!data.TryGetValue("password", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "password",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "password",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["password"] = value;
	if (value.Length < 8)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "password",
			Message = "Password must be at least 8 characters long."
		});
		flag = false;
	}
	if (value.Length > 100)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "password",
			Message = "Password must be at most 100 characters long."
		});
		flag = false;
	}
	if (flag)
	{
		successData["password"] = data["password"];
	}
}
			}
			{
				flag = true;
				if (!data.TryGetValue("profile", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "profile",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string filePath)
{
	result.Errors.Add(new ValidationError
	{
		Path = "profile",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	if (!System.IO.File.Exists(filePath))
	{
		result.Errors.Add(new ValidationError
		{
			Path = "profile",
			Message = "File does not exist."
		});
		flag = false;
	}
	else
	{
		if (!FileSignatureValidator.ValidateExtension(filePath, ["jpg","jpeg","png","webp","avif","heic","heif"]))
		{
			result.Errors.Add(new ValidationError
			{
				Path = "profile",
				Message = "Invalid value."
			});
			flag = false;
		}

		if (flag)
		{
			var dimensions = ImageHelper.GetDimensions(filePath);
			if (dimensions.Width != 512)
			{
				result.Errors.Add(new ValidationError
				{
					Path = "profile",
					Message = "Invalid image width"
				});
				flag = false;
			}
			if (dimensions.Height != 512)
			{
				result.Errors.Add(new ValidationError
				{
					Path = "profile",
					Message = "Invalid image height"
				});
				flag = false;
			}
		}

		if (flag)
		{
			successData["profile"] = data["profile"];
		}
	}
}

			}
			{
				flag = true;
				if (!data.TryGetValue("username", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "username",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "username",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["username"] = value;
	if (value.Length < 3)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "username",
			Message = "Username must be at least 3 characters long."
		});
		flag = false;
	}
	if (value.Length > 20)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "username",
			Message = "Username must be at most 20 characters long."
		});
		flag = false;
	}
	if (flag)
	{
		successData["username"] = data["username"];
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
		public static async Task<ValidationResult> ValidateRegister(Dictionary<string, object?> data)
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
				if (!data.TryGetValue("email", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "email",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "email",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["email"] = value;
	if (flag)
	{
		successData["email"] = data["email"];
	}
}
			}
			{
				flag = true;
				if (!data.TryGetValue("password", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "password",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "password",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["password"] = value;
	if (value.Length < 8)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "password",
			Message = "Password must be at least 8 characters long."
		});
		flag = false;
	}
	if (value.Length > 100)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "password",
			Message = "Password must be at most 100 characters long."
		});
		flag = false;
	}
	if (flag)
	{
		successData["password"] = data["password"];
	}
}
			}
			{
				flag = true;
				if (!data.TryGetValue("username", out var rawValue) || rawValue == null)
{
	result.Errors.Add(new ValidationError
	{
		Path = "username",
		Message = "Invalid value."
	});
	flag = false;
}
else if (rawValue is not string value)
{
	result.Errors.Add(new ValidationError
	{
		Path = "username",
		Message = "Invalid value."
	});
	flag = false;
}
else
{
	data["username"] = value;
	if (value.Length < 3)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "username",
			Message = "Username must be at least 3 characters long."
		});
		flag = false;
	}
	if (value.Length > 20)
	{
		result.Errors.Add(new ValidationError
		{
			Path = "username",
			Message = "Username must be at most 20 characters long."
		});
		flag = false;
	}
	if (flag)
	{
		successData["username"] = data["username"];
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