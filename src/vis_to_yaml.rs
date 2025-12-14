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
    while let Some(line) = rules.next() {
        if line.starts_with("      error") {
            if rule.is_empty() {
                // validates if the error is written without a rule before it
                eprintln!("Error is put without a rule");
                continue;
            }
            let prev_rule = rule.split_once(':').unwrap();
            let error_msg = &line.split_once(':').unwrap().1.trim().to_string();
            let result = format!(
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
