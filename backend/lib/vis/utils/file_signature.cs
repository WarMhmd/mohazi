using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace Generated.Validation.Utils
{
    public static class FileSignatureValidator
    {
        private static readonly Dictionary<string, byte[][]> FileSignatureMap =
            new(StringComparer.OrdinalIgnoreCase)
            {
                ["jpg"] = new[]
                {
                    new byte[] { 0xFF, 0xD8, 0xFF }
                },
                ["jpeg"] = new[]
                {
                    new byte[] { 0xFF, 0xD8, 0xFF }
                },
                ["png"] = new[]
                {
                    new byte[] { 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A }
                },
                ["gif"] = new[]
                {
                    new byte[] { 0x47, 0x49, 0x46, 0x38, 0x37, 0x61 },
                    new byte[] { 0x47, 0x49, 0x46, 0x38, 0x39, 0x61 }
                },
                ["webp"] = new[]
                {
                    new byte[] { 0x52, 0x49, 0x46, 0x46 }
                },
                ["pdf"] = new[]
                {
                    new byte[] { 0x25, 0x50, 0x44, 0x46 }
                },
                ["zip"] = new[]
                {
                    new byte[] { 0x50, 0x4B, 0x03, 0x04 }
                }
            };

        public static async Task<bool> ValidateFileSignatureAsync(FileInput fileObj, string fileExt)
        {
            var signatures = GetSignaturesByExtension(fileExt);

            if (signatures == null)
                return true;

            if (fileObj == null)
                return false;

            var maxSignatureLength = signatures.Max(signature => signature.Length);
            var headerBytes = await ReadFileHeaderAsync(fileObj, maxSignatureLength);

            return MatchesSignature(headerBytes, signatures);
        }

        private static byte[][] GetSignaturesByExtension(string ext)
        {
            if (string.IsNullOrWhiteSpace(ext))
                return null;

            return FileSignatureMap.TryGetValue(ext, out var signatures)
                ? signatures
                : null;
        }

        private static bool MatchesSignature(byte[] headerBytes, byte[][] signatures)
        {
            if (headerBytes == null || signatures == null)
                return false;

            return signatures.Any(signature =>
            {
                if (headerBytes.Length < signature.Length)
                    return false;

                for (var i = 0; i < signature.Length; i++)
                {
                    if (headerBytes[i] != signature[i])
                        return false;
                }

                return true;
            });
        }

        private static async Task<byte[]> ReadFileHeaderAsync(FileInput fileObj, int length)
        {
            using var stream = fileObj.OpenReadStream();

            var headerBytes = new byte[length];
            var totalRead = 0;

            while (totalRead < length)
            {
                var bytesRead = await stream.ReadAsync(headerBytes, totalRead, length - totalRead);

                if (bytesRead == 0)
                    break;

                totalRead += bytesRead;
            }

            if (totalRead == length)
                return headerBytes;

            return headerBytes.Take(totalRead).ToArray();
        }
    }
}