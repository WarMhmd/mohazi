using System;
using System.Collections.Generic;
using System.Threading.Tasks;

using Generated.Validation.Utils;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateBase64_form(Dictionary<string, object?> data)
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

if (!data.TryGetValue("standard", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "standard",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "standard",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    if (!Base64Validator.IsStrictBase64(value, false))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "standard",
            Message = "Invalid value."
        });
        flag = false;
    }

    if (flag)
    {
        if (flag)
        {
            byte[] decodedBytes = Convert.FromBase64String(value.Replace('-', '+').Replace('_', '/'));
            if (decodedBytes.Length < 10)
            {
                result.Errors.Add(new ValidationError
                {
                    Path = "standard",
                    Message = "Base64 too small"
                });
                flag = false;
            }
        }
    }
    data["standard"] = value;
}
if (flag)
                {
                    successData["standard"] = data["standard"];
                }
            }
            {
                flag = true;bool flag = true;

if (!data.TryGetValue("url_safe", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "url_safe",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "url_safe",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    if (!Base64Validator.IsStrictBase64(value, true))
    {
        result.Errors.Add(new ValidationError
        {
            Path = "url_safe",
            Message = "Invalid value."
        });
        flag = false;
    }

    if (flag)
    {
        bool hasPadding = value.EndsWith("=");
        if (hasPadding != false)
        {
            result.Errors.Add(new ValidationError
            {
                Path = "url_safe",
                Message = "Invalid base64 padding"
            });
            flag = false;
        }
    }
    value = value.Trim();
    data["url_safe"] = value;
}
if (flag)
                {
                    successData["url_safe"] = data["url_safe"];
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