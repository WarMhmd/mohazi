use std::{fs, io::Write};

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
        if line.trim().starts_with("error") && !rule.trim().starts_with("value") {
            if rule.is_empty() {
                // validates if the error is written without a rule before it
                eprintln!("Err: No rules provided");
                continue;
            }
            let number_of_spaces = line.find("error").unwrap();
            let (rule_name, rule_value) = rule.split_once(':').unwrap();
            let error_msg = &line.split_once(':').unwrap().1.trim().to_string();

            let result = format!(
                "{}{}: {{ \"value\": {}, \"error\": {} }}\n",
                String::from(" ").repeat(number_of_spaces),
                rule_name.trim(),
                rule_value.trim(),
                error_msg
            );

            let mut trimmed_end = final_rules.lines().collect::<Vec<&str>>();
            trimmed_end.pop(); // this prevents the repetition of rules
            trimmed_end.push(&result);
            final_rules = trimmed_end.join("\n");
        } else {
            final_rules.push_str(line.as_str());
            final_rules.push_str("\n");
        }
        rule = line;
    }

    final_rules
}

/// This function is supposed to read the .vis input file, convert the custom formats into valid
/// yaml and then write it to the output file in yaml extension
pub fn read_vis_file(vis_path: &str) {
    let vis_file = fs::read_to_string(vis_path).unwrap();
    let mut output_file = fs::File::create("output.yaml").unwrap();
    let mut lines = vis_file.lines();
    let mut rules_batch = String::new();
    let mut writing_rules = false;
    let mut collecting_rules = false;
    let mut rules_spaces = 0;
    while let Some(mut line) = lines.next() {
        if line.trim().starts_with("rules:") {
            rules_spaces = line.find("rules:").unwrap();
            collecting_rules = true;
            output_file.write_all(line.as_bytes()).unwrap();
            output_file.write_all(b"\n").unwrap();
            continue;
        }

        while collecting_rules {
            if !line.starts_with(format!("{} ", " ".repeat(rules_spaces)).as_str()) {
                collecting_rules = false;
                writing_rules = true;
            }
            rules_batch.push_str(line);
            rules_batch.push_str("\n");
            line = lines
                .next()
                .or_else(|| {
                    collecting_rules = false;
                    Some("EOF")
                })
                .unwrap();
        }
        if line == "EOF" {
            break;
        }

        if writing_rules {
            let rules = handle_custom_rule_format(rules_batch.clone());
            output_file.write_all(rules.as_bytes()).unwrap();
            rules_batch = String::new();
            writing_rules = false;
        }
        output_file.write_all(line.as_bytes()).unwrap();
        output_file.write_all(b"\n").unwrap();
    }

    if !rules_batch.is_empty() {
        let rules = handle_custom_rule_format(rules_batch.clone());
        output_file.write_all(rules.as_bytes()).unwrap();
    }
}
