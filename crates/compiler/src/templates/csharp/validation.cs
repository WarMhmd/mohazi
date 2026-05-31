using System.Collections.Generic;

namespace Generated.Validation
{
    public class ValidationResult
    {
        public bool Success { get; set; }
        public List<ValidationError> Errors { get; set; } = new List<ValidationError>();
        public Dictionary<string, object?>? Data { get; set; }
    }

    public class ValidationError
    {
        public string Path { get; set; } = string.Empty;
        public string Message { get; set; } = string.Empty;
    }
}