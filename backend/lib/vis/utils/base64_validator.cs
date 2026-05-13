using System;

namespace Generated.Validation.Utils
{
    public static class Base64Validator
    {
        public static bool IsStrictBase64(string str, bool url = false)
        {
            if (string.IsNullOrEmpty(str) || str.Length % 4 != 0)
                return false;

            try
            {
                string base64 = str;
                if (url)
                {
                    base64 = str.Replace('-', '+').Replace('_', '/');
                }

                byte[] bytes = Convert.FromBase64String(base64);
                string roundTrip = Convert.ToBase64String(bytes);
                
                if (url) {
                   roundTrip = roundTrip.Replace('+', '-').Replace('/', '_').TrimEnd('=');
                   // Wait, Base64Url often omits padding. But the regex says it can have it.
                   // Let's re-check the regex: ^(?:[A-Za-z0-9_-]{4})*(?:[A-Za-z0-9_-]{2}==|[A-Za-z0-9_-]{3}=)?$
                   // This regex REQUIRES padding if it's not a multiple of 4? 
                   // No, (?:...)? makes it optional.
                   // Actually, if it's exactly multiple of 4, it might have padding.
                }

                return roundTrip == base64; // Compare with normalized base64 for URL safe
            }
            catch
            {
                return false;
            }
        }
    }
}
