using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Threading.Tasks;

using Generated.Validation.Utils;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateBirth(Dictionary<string, object?> data)
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
                flag = true;bool flag = true;

if (!data.TryGetValue("childName", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "childName",
        Message = "\"Child name is invalid\""
    });
    flag = false;
}
else if (rawValue is JsonElement stringJe)
{
<<<<<<< HEAD
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
=======
    result.Errors.Add(new ValidationError
    {
        Path = "childName",
        Message = "\"Child name is invalid\""
    });
    flag = false;
>>>>>>> Alhareth-types
}
else if (rawValue is string value)
{
<<<<<<< HEAD
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
=======
    data["childName"] = value;
    if (value.Length < 3)
    {
        result.Errors.Add(new ValidationError
        {
            Path = "childName",
            Message = "Child name cannot be shorter than 3 characters."
        });
        flag = false;
    }
    if (value.Length > 20)
    {
        result.Errors.Add(new ValidationError
        {
            Path = "childName",
            Message = "Child name cannot be longer than 20 characters."
        });
        flag = false;
    }
}if (flag)
                {
                    successData["childName"] = data["childName"];
                }
            }
            {
                flag = true;bool flag = true;

if (!data.TryGetValue("dateOfBirth", out var rawValue) || rawValue == null)
>>>>>>> Alhareth-types
{
    result.Errors.Add(new ValidationError
    {
        Path = "dateOfBirth",
        Message = "\"Invalid date of birth\""
    });
    flag = false;
}
else if (rawValue is JsonElement stringJe)
{
<<<<<<< HEAD
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
=======
    result.Errors.Add(new ValidationError
    {
        Path = "dateOfBirth",
        Message = "\"Invalid date of birth\""
    });
    flag = false;
>>>>>>> Alhareth-types
}
else if (rawValue is string value)
{
<<<<<<< HEAD
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
=======
    data["dateOfBirth"] = value;

    if (flag)
    {
        if (!DateTime.TryParse(value, out DateTime dateObj))
        {
            result.Errors.Add(new ValidationError
            {
                Path = "dateOfBirth",
                Message = "\"Invalid date of birth\""
            });
            flag = false;
        }
        else
        {
            
            if (flag)
            {
                string formatStr = "YYYY-mm-dd";
                formatStr = formatStr.Replace("DDD", dateObj.DayOfYear.ToString("000"));
                formatStr = formatStr.Replace("YYYY", "yyyy").Replace("YY", "yy");
                formatStr = formatStr.Replace("a", "tt");
                data["dateOfBirth"] = dateObj.ToString(formatStr);
            }
        }
    }
}
if (flag)
                {
                    successData["dateOfBirth"] = data["dateOfBirth"];
                }
            }
            {
                flag = true;bool flag = true;
>>>>>>> Alhareth-types

if (!data.TryGetValue("id", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "id",
        Message = "\"Invalid UUID\""
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
        Path = "id",
        Message = "\"Invalid UUID\""
    });
    flag = false;
}
<<<<<<< HEAD
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
=======
else
{
    data["id"] = value;
    if (value.Length != 36)
    {
        result.Errors.Add(new ValidationError
        {
            Path = "id",
            Message = "\"Invalid UUID\""
        });
        flag = false;
    }
    if (!System.Text.RegularExpressions.Regex.IsMatch(value, @"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$"))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "id",
            Message = "\"Invalid UUID\""
        });
        flag = false;
    }
}
if (flag)
                {
                    successData["id"] = data["id"];
                }
            }
>>>>>>> Alhareth-types

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