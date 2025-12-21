use crate::ast::{Field, FieldType, Form, NumberRules, RuleType, Rules, StringRules};
use indexmap::IndexMap;
use std::collections::HashMap;

/// Helper function
fn flush_rules(builder: &mut ActiveRuleBuilder, current_field: &mut Field) {
    match std::mem::replace(builder, ActiveRuleBuilder::None) {
        ActiveRuleBuilder::String(r) => current_field.rules.push(Rules::String(r)),
        ActiveRuleBuilder::Number(r) => current_field.rules.push(Rules::Number(r)),
        ActiveRuleBuilder::None => {}
    }
}

enum ActiveRuleBuilder {
    String(StringRules),
    Number(NumberRules),
    None,
}

// 1. Define the Level Enum
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Level {
    Form,
    Field,
    Property,
    RulesAndTransformations,
    ValueAndErrorPair,
}

// Helper to convert integer depth to Enum
impl Level {
    fn from_depth(depth: usize) -> Result<Level, String> {
        match depth {
            0 => Ok(Level::Form),
            1 => Ok(Level::Field),
            2 => Ok(Level::Property),
            3 => Ok(Level::RulesAndTransformations),
            4 => Ok(Level::ValueAndErrorPair),
            n => Err(format!("Nesting too deep: level {}", n)),
        }
    }

    fn get_next_level(&self) -> Result<Level, String> {
        match *self {
            Level::Form => Ok(Level::Field),
            Level::Field => Ok(Level::Property),
            Level::Property => Ok(Level::RulesAndTransformations),
            Level::RulesAndTransformations => Ok(Level::ValueAndErrorPair),
            Level::ValueAndErrorPair => Err("Error: Nesting too deep".to_string()),
        }
    }

    fn get_level_from_index(i: usize) -> Result<Level, String> {
        match i {
            0 => Ok(Level::Form),
            1 => Ok(Level::Field),
            2 => Ok(Level::Property),
            3 => Ok(Level::RulesAndTransformations),
            4 => Ok(Level::ValueAndErrorPair),
            _ => Err("Invalid level Index".to_string()),
        }
    }
}

pub fn raw_spaces(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

pub fn parse_vis(input: &str) -> Result<IndexMap<String, Form>, Vec<String>> {
    let mut forms: IndexMap<String, Form> = IndexMap::new();
    let mut errors: Vec<String> = Vec::new();

    let mut current_level = Level::Form;
    let mut levels_vector = vec![0; 5]; // this vector stores the level depth and is indexed

    let mut parsing_type = FieldType::String;
    // through the Level enum
    let mut prev_spaces = 0;
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#')) // ignores empty lines and
        // lines starting with #
        .map(|l| {
            // strips comments from the line
            if let Some(line_with_comment) = l.split_once('#') {
                line_with_comment.0 // the part without the comment
            } else {
                l
            }
        })
        .collect();

    let mut iter = lines.into_iter().peekable();

    let mut current_form_name = String::new();
    let mut current_field_name = String::new();
    let mut active_context = "none"; // transform or rule
    let mut active_rule_builder = ActiveRuleBuilder::None;
    let mut prev_level = Level::Form;

    while let Some(line) = iter.next() {
        let current_spaces = raw_spaces(line);
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
        let key = parts[0].trim();
        let value = if parts.len() > 1 { parts[1].trim() } else { "" };

        if current_spaces > prev_spaces {
            current_level = current_level.get_next_level().expect("Nested too deep");
            levels_vector[current_level as usize] = current_spaces;
        } else if current_spaces < prev_spaces {
            let new_level_idx = levels_vector
                .iter()
                .position(|&v| v == current_spaces)
                .expect("Indentation mismatch: Level not found");
            let new_level = Level::get_level_from_index(new_level_idx).unwrap();

            if prev_level == Level::RulesAndTransformations
                && new_level != Level::RulesAndTransformations
            {
                // We need to borrow current_field here to save the rules
                // (Using a separate scope or cell logic might be needed if borrow checker complains,
                // but since we get the mutable reference fresh below, we can do it after this block if careful,
                // or right here if we already have the reference.)

                // EASIER: Just flush at the end of the previous iteration?
                // No, we only know we finished the block NOW.

                // Let's re-acquire the field to flush.
                if let Some(form) = forms.get_mut(&current_form_name) {
                    if let Some(field) = form.fields.get_mut(&current_field_name) {
                        flush_rules(&mut active_rule_builder, field);
                    }
                }
            }
            if new_level == Level::Field {
                if let Some(form) = forms.get_mut(&current_form_name) {
                    if let Some(finished_field) = form.fields.get_mut(&current_field_name) {
                        // 1. Take the rules out (empties the field's vector)
                        let raw_rules = std::mem::take(&mut finished_field.rules);

                        // 2. Prepare accumulators
                        let mut merged_string = StringRules::new();
                        let mut has_string = false;

                        let mut merged_number = NumberRules::new();
                        let mut has_number = false;

                        // 3. Iterate and Merge
                        for rule in raw_rules {
                            match rule {
                                Rules::String(r) => {
                                    has_string = true;
                                    merged_string.merge(r, &mut errors);
                                }
                                Rules::Number(r) => {
                                    has_number = true;
                                    merged_number.merge(r, &mut errors);
                                }
                            }
                        }

                        // 4. Push back the merged results
                        if has_string {
                            println!("Has string");
                            finished_field.rules.push(Rules::String(merged_string));
                        }
                        if has_number {
                            finished_field.rules.push(Rules::Number(merged_number));
                        }
                    }
                }
            }

            current_level = new_level;
        }

        prev_spaces = current_spaces;
        prev_level = current_level;

        match current_level {
            Level::Form => {
                current_form_name = key.to_string();
                if !value.is_empty() {
                    errors.push(format!(
                        "Error: Form {} cannot be set to a single value",
                        key
                    ));
                    continue;
                }

                if forms.contains_key(key) {
                    errors.push(format!("Duplicate Form name: {}", key));
                    continue;
                }

                forms.insert(
                    current_form_name.clone(),
                    Form {
                        fields: HashMap::new(),
                    },
                );
            }
            Level::Field => {
                let current_form = forms.get_mut(&current_form_name).unwrap();
                current_field_name = key.to_string();

                if !value.is_empty() {
                    errors.push(format!(
                        "Error: Field {} cannot be set to a single value",
                        key
                    ));
                    continue;
                }

                if current_form.fields.contains_key(key) {
                    errors.push(format!("Duplicate Field name: {}", key));
                    continue;
                }

                current_form.fields.insert(
                    current_field_name.clone(),
                    Field::default(), // this would be filled with the appropriate data later
                );

                parsing_type = FieldType::String;
            }
            Level::Property => {
                let property_name = key;
                let current_form = forms.get_mut(&current_form_name).unwrap();
                let current_field = current_form.fields.get_mut(&current_field_name).unwrap();

                match property_name {
                    // match all possible properties
                    "type" | "fieldType" => {
                        if value.is_empty() {
                            errors.push(format!("Error: field type cannot be empty"));
                            continue;
                        }

                        let field_type: FieldType = match value {
                            "string" => FieldType::String,
                            "number" => FieldType::Number,
                            "boolean" => FieldType::Boolean,
                            "array" => FieldType::Array,
                            _ => {
                                errors.push(format!("Unknown field type {}", value));
                                continue;
                            }
                        };

                        current_field.field_type = field_type;
                        parsing_type = field_type;
                    }
                    "required" => {
                        if value.is_empty() {
                            errors.push(format!("Error: required cannot be empty"));
                            continue;
                        }

                        match value {
                            "true" => current_field.required = true,
                            "false" => current_field.required = false,
                            _ => {
                                errors.push(format!("Unknown required value {}", value));
                                continue;
                            }
                        };
                    }
                    "defaultError" => {
                        current_field.default_error = Some(value.to_string());
                    }
                    "rules" => {
                        if !value.is_empty() {
                            errors.push(format!("Error: rules cannot be set to a single value"));
                            continue;
                        }

                        active_context = "rules";
                    }
                    "transform" => {
                        if !value.is_empty() {
                            errors.push(format!("Error: rules cannot be set to a single value"));
                            continue;
                        }

                        active_context = "transform";
                    }
                    _ => {
                        errors.push(format!(
                            "Unknown property {} at {}",
                            property_name, &current_field_name
                        ));
                    }
                }
            }
            Level::RulesAndTransformations => match active_context {
                "rules" => {
                    let current_form = forms.get_mut(&current_form_name).unwrap();
                    let current_field = current_form.fields.get_mut(&current_field_name).unwrap();

                    // A. INITIALIZE BUILDER
                    if matches!(active_rule_builder, ActiveRuleBuilder::None) {
                        active_rule_builder = match parsing_type {
                            FieldType::Number => ActiveRuleBuilder::Number(NumberRules::new()),
                            _ => ActiveRuleBuilder::String(StringRules::new()),
                        };
                    }

                    // B. PARSE (Variations 1, 2, 3)
                    let (final_val, final_err): (serde_yaml_ng::Value, Option<String>) = {
                        if value.starts_with('{') {
                            // Variation 1: Inline JSON
                            let rt: RuleType<serde_yaml_ng::Value> = serde_yaml_ng::from_str(value)
                                .map_err(|e| vec![format!("Invalid inline rule: {}", e)])?;
                            (rt.value, rt.error)
                        } else if value.is_empty() {
                            // Variation 3: Nested Block (Lookahead)
                            let mut n_val = serde_yaml_ng::Value::Null;
                            let mut n_err = None;

                            while let Some(peek_line) = iter.peek() {
                                let p_spaces = raw_spaces(peek_line);
                                // Stop if indentation is not deeper (Level 4)
                                if p_spaces <= current_spaces {
                                    break;
                                }

                                let child_line = iter.next().unwrap();
                                let c_parts: Vec<&str> = child_line.trim().splitn(2, ':').collect();
                                let c_key = c_parts[0].trim();
                                let c_val = if c_parts.len() > 1 {
                                    c_parts[1].trim()
                                } else {
                                    ""
                                };

                                match c_key {
                                    "value" => {
                                        n_val = serde_yaml_ng::from_str(c_val).unwrap_or(
                                            serde_yaml_ng::Value::String(c_val.to_string()),
                                        )
                                    }
                                    "error" => {
                                        n_err = Some(c_val.replace("'", "").replace("\"", ""))
                                    }
                                    _ => {}
                                }
                            }
                            (n_val, n_err)
                        } else {
                            // Variation 2: Value + Sibling Error
                            let clean_val_str = value.trim().trim_end_matches(',');
                            let s_val: serde_yaml_ng::Value =
                                serde_yaml_ng::from_str(clean_val_str)
                                    .unwrap_or(serde_yaml_ng::Value::String(value.to_string()));
                            let mut s_err = None;

                            // Peek for sibling error
                            if let Some(peek_line) = iter.peek() {
                                let p_spaces = raw_spaces(peek_line);
                                if p_spaces == current_spaces {
                                    let p_trimmed = peek_line.trim();
                                    if p_trimmed.starts_with("error:") {
                                        let err_parts: Vec<&str> =
                                            p_trimmed.splitn(2, ':').collect();
                                        s_err = Some(
                                            err_parts[1].trim().replace("'", "").replace("\"", ""),
                                        );
                                        iter.next(); // Consume the error line!
                                    }
                                }
                            }
                            (s_val, s_err)
                        }
                    };

                    // C. SET RULE
                    let result = match &mut active_rule_builder {
                        ActiveRuleBuilder::String(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Number(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::None => Ok(()),
                    };

                    if let Err(msg) = result {
                        errors.push(format!("Rule Error at {}: {}", current_field_name, msg));
                    }
                }
                "transform" => {
                    let current_form = forms.get_mut(&current_form_name).unwrap();
                    let current_field = current_form.fields.get_mut(&current_field_name).unwrap();

                    match key {
                        "cast" => {
                            // 1. Parse the target type
                            let cast_type = match value {
                                "string" => FieldType::String,
                                "number" => FieldType::Number,
                                "boolean" => FieldType::Boolean,
                                "array" => FieldType::Array,
                                _ => {
                                    errors.push(format!("Invalid cast type '{}'", value));
                                    continue;
                                }
                            };

                            // 2. Update the field type immediately
                            parsing_type = cast_type;

                            // 3. CRITICAL: Reset the builder if it doesn't match the new type!
                            // If we defined some string rules, then cast to number,
                            // we need to ensure the next rule we parse uses a NumberBuilder.

                            match (&active_rule_builder, parsing_type) {
                                (ActiveRuleBuilder::String(_), FieldType::Number) => {
                                    // Flush existing string rules before switching?
                                    // Or just drop them? usually flushing is safer.
                                    flush_rules(&mut active_rule_builder, current_field);
                                    active_rule_builder = ActiveRuleBuilder::None;
                                }
                                (ActiveRuleBuilder::Number(_), FieldType::String) => {
                                    flush_rules(&mut active_rule_builder, current_field);
                                    active_rule_builder = ActiveRuleBuilder::None;
                                }
                                _ => {} // No conflict or builder not started yet
                            }
                        }
                        _ => {
                            errors.push(format!("Unknown transform property: {}", key));
                        }
                    }
                }
                _ => {
                    errors.push(format!(
                        "Unknown context {} at {}",
                        active_context, &current_field_name
                    ));
                }
            },
            Level::ValueAndErrorPair => todo!(),
        }
        prev_spaces = current_spaces;
    }

    // FINAL FLUSH (Important! Catch the last block if file ends with rules)
    if let Some(form) = forms.get_mut(&current_form_name) {
        if let Some(field) = form.fields.get_mut(&current_field_name) {
            flush_rules(&mut active_rule_builder, field);
        }
    }

    // collect rules togather

    if errors.is_empty() {
        Ok(forms)
    } else {
        Err(errors)
    }
}

// /// This function handles the way we write custom rules and errors and converts them to valid yaml
// /// Example:
// ///     min_length: 5
// ///     error: "Password must be at least 5 characters long"
// ///     max_length: 10
// ///     error: "Password must be at most 10 characters long"
// ///
// /// Should become like this:
// ///     min_length: { "value": 5, "error": "Password must be at least 5 characters long" }
// ///     max_length: { "value": 10, "error": "Password must be at most 5 characters long" }
// ///
// /// PAY ATTENTION TO THE SPACES
// pub fn handle_custom_rule_format(rules: String) -> String {
//     let mut rules = rules.lines().map(|rule| rule.to_string());
//     let mut rule = String::new();
//     let mut final_rules = String::new();
//     while let Some(line) = rules.next() {
//         if line.trim().starts_with("error") && !rule.trim().starts_with("value") {
//             if rule.is_empty() {
//                 // validates if the error is written without a rule before it
//                 eprintln!("Err: No rules provided");
//                 continue;
//             }
//             let number_of_spaces = line.find("error").unwrap();
//             let (rule_name, rule_value) = rule.split_once(':').unwrap();
//             let error_msg = &line.split_once(':').unwrap().1.trim().to_string();
//
//             let result = format!(
//                 "{}{}: {{ \"value\": {}, \"error\": {} }}\n",
//                 String::from(" ").repeat(number_of_spaces),
//                 rule_name.trim(),
//                 rule_value.trim(),
//                 error_msg
//             );
//
//             let mut trimmed_end = final_rules.lines().collect::<Vec<&str>>();
//             trimmed_end.pop(); // this prevents the repetition of rules
//             trimmed_end.push(&result);
//             final_rules = trimmed_end.join("\n");
//         } else {
//             final_rules.push_str(line.as_str());
//             final_rules.push_str("\n");
//         }
//         rule = line;
//     }
//
//     final_rules
// }
//
// /// This function is supposed to read the .vis input file, convert the custom formats into valid
// /// yaml and then write it to the output file in yaml extension
// pub fn read_vis_file(vis_path: &str) {
//     let vis_file = fs::read_to_string(vis_path).unwrap();
//     let mut output_file = fs::File::create("output.yaml").unwrap();
//     let mut lines = vis_file.lines();
//     let mut rules_batch = String::new();
//     let mut writing_rules = false;
//     let mut collecting_rules = false;
//     let mut rules_spaces = 0;
//     while let Some(mut line) = lines.next() {
//         if line.trim().starts_with("rules:") {
//             rules_spaces = line.find("rules:").unwrap();
//             collecting_rules = true;
//             output_file.write_all(line.as_bytes()).unwrap();
//             output_file.write_all(b"\n").unwrap();
//             continue;
//         }
//
//         while collecting_rules {
//             if !line.starts_with(format!("{} ", " ".repeat(rules_spaces)).as_str()) {
//                 collecting_rules = false;
//                 writing_rules = true;
//             }
//             rules_batch.push_str(line);
//             rules_batch.push_str("\n");
//             line = lines
//                 .next()
//                 .or_else(|| {
//                     collecting_rules = false;
//                     Some("EOF")
//                 })
//                 .unwrap();
//         }
//         if line == "EOF" {
//             break;
//         }
//
//         if writing_rules {
//             let rules = handle_custom_rule_format(rules_batch.clone());
//             output_file.write_all(rules.as_bytes()).unwrap();
//             rules_batch = String::new();
//             writing_rules = false;
//         }
//         output_file.write_all(line.as_bytes()).unwrap();
//         output_file.write_all(b"\n").unwrap();
//     }
//
//     if !rules_batch.is_empty() {
//         let rules = handle_custom_rule_format(rules_batch.clone());
//         output_file.write_all(rules.as_bytes()).unwrap();
//     }
// }
