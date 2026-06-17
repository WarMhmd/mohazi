using System;
using System.Collections.Generic;
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
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "childName",
        Message = "\"Child name is invalid\""
    });
    flag = false;
}
else
{
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
{
    result.Errors.Add(new ValidationError
    {
        Path = "dateOfBirth",
        Message = "\"Invalid date of birth\""
    });
    flag = false;
}
else if (rawValue is not string value)
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

if (!data.TryGetValue("id", out var rawValue) || rawValue == null)
{
    result.Errors.Add(new ValidationError
    {
        Path = "id",
        Message = "\"Invalid UUID\""
    });
    flag = false;
}
else if (rawValue is not string value)
{
    result.Errors.Add(new ValidationError
    {
        Path = "id",
        Message = "\"Invalid UUID\""
    });
    flag = false;
}
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