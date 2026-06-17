using System;
using System.Collections.Generic;
using System.Threading.Tasks;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateCuid2_form(Dictionary<string, object?> data)
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

if (!data.TryGetValue("id", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "id",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "id",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    if (value.Length < 10)
    {
        result.Errors.Add(new ValidationError
        {
            Path = "id",
            Message = "Invalid value."
        });
        flag = false;
    }
    if (value.Length > 31)
    {
        result.Errors.Add(new ValidationError
        {
            Path = "id",
            Message = "Invalid value."
        });
        flag = false;
    }
    if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^c[a-z0-9]+$"))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "id",
            Message = "Invalid value."
        });
        flag = false;
    }
    value = value.Trim();
    data["id"] = value;
}
if (flag)
                {
                    successData["id"] = data["id"];
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