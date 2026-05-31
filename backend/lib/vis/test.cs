using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Threading.Tasks;

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
else if (rawValue is JsonElement stringJe)
{
	if (stringJe.ValueKind != JsonValueKind.String)
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
		var value = stringJe.GetString()!;
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
		if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$"))
		{
			result.Errors.Add(new ValidationError
			{
				Path = "password",
				Message = "Password must contain at least one letter and one number."
			});
			flag = false;
		}
		if (flag)
		{
			successData["password"] = data["password"];
		}
	}
}
else if (rawValue is string value)
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
	if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$"))
	{
		result.Errors.Add(new ValidationError
		{
			Path = "password",
			Message = "Password must contain at least one letter and one number."
		});
		flag = false;
	}
	if (flag)
	{
		successData["password"] = data["password"];
	}
}
else
{
	result.Errors.Add(new ValidationError
	{
		Path = "password",
		Message = "Invalid value."
	});
	flag = false;
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
else if (rawValue is JsonElement stringJe)
{
	if (stringJe.ValueKind != JsonValueKind.String)
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
		var value = stringJe.GetString()!;
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
		if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[a-zA-Z0-9_]+$"))
		{
			result.Errors.Add(new ValidationError
			{
				Path = "username",
				Message = "Username can only contain letters, numbers, and underscores."
			});
			flag = false;
		}
		if (flag)
		{
			successData["username"] = data["username"];
		}
	}
}
else if (rawValue is string value)
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
	if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[a-zA-Z0-9_]+$"))
	{
		result.Errors.Add(new ValidationError
		{
			Path = "username",
			Message = "Username can only contain letters, numbers, and underscores."
		});
		flag = false;
	}
	if (flag)
	{
		successData["username"] = data["username"];
	}
}
else
{
	result.Errors.Add(new ValidationError
	{
		Path = "username",
		Message = "Invalid value."
	});
	flag = false;
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
else if (rawValue is JsonElement stringJe)
{
	if (stringJe.ValueKind != JsonValueKind.String)
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
		var value = stringJe.GetString()!;
		data["email"] = value;
		if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$"))
		{
			result.Errors.Add(new ValidationError
			{
				Path = "email",
				Message = "Please enter a valid email address."
			});
			flag = false;
		}
		if (flag)
		{
			successData["email"] = data["email"];
		}
	}
}
else if (rawValue is string value)
{
	data["email"] = value;
	if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$"))
	{
		result.Errors.Add(new ValidationError
		{
			Path = "email",
			Message = "Please enter a valid email address."
		});
		flag = false;
	}
	if (flag)
	{
		successData["email"] = data["email"];
	}
}
else
{
	result.Errors.Add(new ValidationError
	{
		Path = "email",
		Message = "Invalid value."
	});
	flag = false;
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
else if (rawValue is JsonElement stringJe)
{
	if (stringJe.ValueKind != JsonValueKind.String)
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
		var value = stringJe.GetString()!;
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
		if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$"))
		{
			result.Errors.Add(new ValidationError
			{
				Path = "password",
				Message = "Password must contain at least one letter and one number."
			});
			flag = false;
		}
		if (flag)
		{
			successData["password"] = data["password"];
		}
	}
}
else if (rawValue is string value)
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
	if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$"))
	{
		result.Errors.Add(new ValidationError
		{
			Path = "password",
			Message = "Password must contain at least one letter and one number."
		});
		flag = false;
	}
	if (flag)
	{
		successData["password"] = data["password"];
	}
}
else
{
	result.Errors.Add(new ValidationError
	{
		Path = "password",
		Message = "Invalid value."
	});
	flag = false;
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
else if (rawValue is JsonElement stringJe)
{
	if (stringJe.ValueKind != JsonValueKind.String)
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
		var value = stringJe.GetString()!;
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
		if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[a-zA-Z0-9_]+$"))
		{
			result.Errors.Add(new ValidationError
			{
				Path = "username",
				Message = "Username can only contain letters, numbers, and underscores."
			});
			flag = false;
		}
		if (flag)
		{
			successData["username"] = data["username"];
		}
	}
}
else if (rawValue is string value)
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
	if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[a-zA-Z0-9_]+$"))
	{
		result.Errors.Add(new ValidationError
		{
			Path = "username",
			Message = "Username can only contain letters, numbers, and underscores."
		});
		flag = false;
	}
	if (flag)
	{
		successData["username"] = data["username"];
	}
}
else
{
	result.Errors.Add(new ValidationError
	{
		Path = "username",
		Message = "Invalid value."
	});
	flag = false;
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