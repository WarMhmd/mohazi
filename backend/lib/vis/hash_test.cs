using System;
using System.Collections.Generic;
using System.Threading.Tasks;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateHash_form(Dictionary<string, object?> data)
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

if (!data.TryGetValue("password_hash", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "password_hash",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "password_hash",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    string pattern = null;
    pattern = @"^\$argon2(id|i|d)\$v=\d+\$m=\d+,t=\d+,p=\d+\$[A-Za-z0-9+/]+={0,2}\$[A-Za-z0-9+/]+={0,2}$";

    if (pattern != null && !System.Text.RegularExpressions.Regex.IsMatch(value, pattern))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "password_hash",
            Message = "Invalid value."
        });
        flag = false;
    }
    value = value.Trim();
    data["password_hash"] = value;
}if (flag)
                {
                    successData["password_hash"] = data["password_hash"];
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