using System;
using System.Collections.Generic;
using System.Threading.Tasks;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateUsername_form(Dictionary<string, object?> data)
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

if (!data.TryGetValue("profile_name", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "profile_name",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "profile_name",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    if (value.Length < 5)
    {
        result.Errors.Add(new ValidationError
        {
            Path = "profile_name",
            Message = "Invalid value."
        });
        flag = false;
    }
    if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[a-zA-Z0-9](?:[._-]?[a-zA-Z0-9]){2,29}$"))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "profile_name",
            Message = "Invalid value."
        });
        flag = false;
    }
    value = value.Trim();
    data["profile_name"] = value;
}
if (flag)
                {
                    successData["profile_name"] = data["profile_name"];
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