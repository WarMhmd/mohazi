using System;
using System.Collections.Generic;
using System.Threading.Tasks;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateMail_form(Dictionary<string, object?> data)
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

if (!data.TryGetValue("user_email", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "user_email",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "user_email",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    if (!System.Text.RegularExpressions.Regex.IsMatch(value, "^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9]?(?:\\.[a-zA-Z0-9]?)*$"))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "user_email",
            Message = "Invalid value."
        });
        flag = false;
    }

    if (flag)
    {
        var domain = value.Split('@').Last();
        if (!new List<string>(["gmail.com","outlook.com"]).Contains(domain))
        {
            result.Errors.Add(new ValidationError
            {
                Path = "user_email",
                Message = "Email domain not allowed"
            });
            flag = false;
        }
    }
    value = value.Trim();
    data["user_email"] = value;
}if (flag)
                {
                    successData["user_email"] = data["user_email"];
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