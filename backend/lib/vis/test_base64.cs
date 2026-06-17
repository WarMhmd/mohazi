using System;
using System.Collections.Generic;
using System.Threading.Tasks;

using Generated.Validation.Utils;


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
                flag = true;bool flag = true;

if (!data.TryGetValue("avatar", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "avatar",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "avatar",
        Message = "Invalid value."
    });
    flag = false;
}
else if (!Base64Validator.IsStrictBase64(value, false)) {
    result.Errors.Add(new ValidationError {
        Path = "avatar",
        Message = "Invalid value."
    });
    flag = false;
} else {
    byte[] decodedBytes = Convert.FromBase64String(value.Replace('-', '+').Replace('_', '/'));
    if (decodedBytes.Length > 1048576) {
        result.Errors.Add(new ValidationError {
            Path = "avatar",
            Message = "Invalid value."
        });
        flag = false;
    }
}
if (flag)
                {
                    successData["avatar"] = data["avatar"];
                }
            }
            {
                flag = true;bool flag = true;

if (!data.TryGetValue("sessionToken", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "sessionToken",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "sessionToken",
        Message = "Invalid value."
    });
    flag = false;
}
else if (!Base64Validator.IsStrictBase64(value, true)) {
    result.Errors.Add(new ValidationError {
        Path = "sessionToken",
        Message = "Invalid value."
    });
    flag = false;
} else {
    byte[] decodedBytes = Convert.FromBase64String(value.Replace('-', '+').Replace('_', '/'));
    if (decodedBytes.Length < 32) {
        result.Errors.Add(new ValidationError {
            Path = "sessionToken",
            Message = "Invalid value."
        });
        flag = false;
    }
    if (decodedBytes.Length > 64) {
        result.Errors.Add(new ValidationError {
            Path = "sessionToken",
            Message = "Invalid value."
        });
        flag = false;
    }
    value = value.Trim();
    data["sessionToken"] = value;
}
if (flag)
                {
                    successData["sessionToken"] = data["sessionToken"];
                }
            }
            {
                flag = true;bool flag = true;

if (!data.TryGetValue("tags", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "tags",
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not System.Collections.IEnumerable enumerable || rawValue is string)
{
    result.Errors.Add(new ValidationError
    {
        Path = "tags",
        Message = "Invalid value."
    });
    flag = false;
}
else
{
    var list = enumerable.Cast<object?>().ToList();

    for (int i = 0; i < list.Count; i++)
    {
        var previousFlag = flag;
        flag = true;

        var itemPath = "tags" + "[" + i + "]";
        data[itemPath] = list[i];bool flag = true;

if (!data.TryGetValue(itemPath, out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = itemPath,
        Message = "Invalid value."
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = itemPath,
        Message = "Invalid value."
    });
    flag = false;
}
else if (!Base64Validator.IsStrictBase64(value, false)) {
    result.Errors.Add(new ValidationError {
        Path = itemPath,
        Message = "Invalid value."
    });
    flag = false;
} else {
}


        list[i] = data[itemPath];
        data.Remove(itemPath);

        var itemFlag = flag;
        flag = previousFlag && itemFlag;}

    data["tags"] = list;
}if (flag)
                {
                    successData["tags"] = data["tags"];
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