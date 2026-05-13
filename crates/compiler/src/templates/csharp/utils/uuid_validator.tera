using System;

namespace Generated.Validation.Utils
{
    public static class UuidValidator
    {
        public static bool Validate(string uuid)
        {
            return Guid.TryParse(uuid, out _);
        }

        public static int GetVersion(string uuid)
        {
            if (Guid.TryParse(uuid, out Guid guid))
            {
                // In a Guid byte array:
                // Data1: 4 bytes
                // Data2: 2 bytes
                // Data3: 2 bytes (The high 4 bits of the first byte of Data3 is the version)
                // In .NET Guid.ToByteArray() returns Data1, Data2, Data3 in little-endian.
                // Data3 is at bytes 6 and 7. Byte 7 is the high byte.
                byte[] bytes = guid.ToByteArray();
                return (bytes[7] >> 4);
            }
            return -1;
        }
    }
}
