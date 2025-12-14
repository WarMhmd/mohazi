/// This function handles the way we write custom rules and errors and converts them to valid yaml
/// Example:
///     min_length: 5
///     error: "Password must be at least 5 characters long"
///     max_length: 10
///     error: "Password must be at most 10 characters long"
///
/// Should become like this:
///     min_length: { "value": 5, "error": "Password must be at least 5 characters long" }
///     max_length: { "value": 10, "error": "Password must be at most 5 characters long" }
///
/// PAY ATTENTION TO THE SPACES
pub fn handle_custom_rule_format(rules: String) -> String {
    let mut rules = rules.lines().map(|rule| rule.to_string());
    let mut rule = String::new();
    let mut final_rules = String::new();
    let mut result = String::new();
    while let Some(line) = rules.next() {
        if line.contains("error") {
            let prev_rule = rule.split_once(':').unwrap();
            let error_msg = &line.split_once(':').unwrap().1.trim().to_string();
            result = format!(
                "      {}: {{ \"value\": {}, \"error\": {} }}\n",
                &prev_rule.0.trim(),
                &prev_rule.1.trim(),
                error_msg
            );
            final_rules.push_str(&result);
        }
        rule = line;
    }

    final_rules
}

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
