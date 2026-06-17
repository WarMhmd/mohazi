using System;
using System.Collections.Generic;
using System.Threading.Tasks;

using Generated.Validation.Utils;


namespace Generated.Validation
{
    public static class Validators
    {
        public static async Task<ValidationResult> ValidateUuid_form(Dictionary<string, object?> data)
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

if (!data.TryGetValue("session_id", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "session_id",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "session_id",
        Message = "Invalid value."
    });
    flag = false;
}
else
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
            Path = "session_id",
            Message = "Invalid UUID version"
        });
        flag = false;
    }
    value = value.Trim();
    data["session_id"] = value;
}
if (flag)
                {
                    successData["session_id"] = data["session_id"];
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