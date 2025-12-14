use crate::vis_to_yaml::handle_custom_rule_format;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length_custom_format() {
        let rules =
            "      min_length: 5\n      error: \"Password must be at least 5 characters long\"";
        let expected = "      min_length: { \"value\": 5, \"error\": \"Password must be at least 5 characters long\" }\n";
        let result = handle_custom_rule_format(rules.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_required_custom_format() {
        let rules =
            "      required: true\n      error: \"Password must be at least 5 characters long\"";
        let expected = "      required: { \"value\": true, \"error\": \"Password must be at least 5 characters long\" }\n";
        let result = handle_custom_rule_format(rules.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_rules_at_once() {
        let rules = "      min_length: 5\n      error: \"Password must be at least 5 characters long\"\n      max_length: 10\n      error: \"Password must be at most 10 characters long\"";
        let expected = "      min_length: { \"value\": 5, \"error\": \"Password must be at least 5 characters long\" }\n      max_length: { \"value\": 10, \"error\": \"Password must be at most 10 characters long\" }\n";
        let result = handle_custom_rule_format(rules.to_string());
        assert_eq!(result, expected);
    }
}
