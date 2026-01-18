use crate::ast::{
    ArrayRules, ArrayTransform, BooleanRules, BooleanTransform, EnumRules, EnumTransform, Field,
    FieldType, FileRules, FileTransform, Form, NumberRules, NumberTransform, Rule, RuleTrait,
    RuleType, StringRules, StringTransform, Transform, TransformTrait,
};
use indexmap::IndexMap;
use std::{collections::HashMap, thread::current};

/// This function take the set of rules that is currently being built and commits it to the actual
/// field
fn flush_rules(builder: &mut ActiveRuleBuilder, current_field: &mut Field) {
    match std::mem::replace(builder, ActiveRuleBuilder::None) {
        ActiveRuleBuilder::String(r) => current_field.rules.push(Rule::String(r)),
        ActiveRuleBuilder::Number(r) => current_field.rules.push(Rule::Number(r)),
        ActiveRuleBuilder::File(r) => current_field.rules.push(Rule::File(r)),
        ActiveRuleBuilder::Boolean(r) => current_field.rules.push(Rule::Boolean(r)),
        ActiveRuleBuilder::Array(r) => current_field.rules.push(Rule::Array(r)),
        ActiveRuleBuilder::Enum(r) => current_field.rules.push(Rule::Enum(r)),
        // todo[Add]: Type
        ActiveRuleBuilder::None => {}
    }
}

/// This function take the set of transforms that is currently being built and commits it to the actual
/// field
fn flush_transforms(builder: &mut ActiveTransformBuilder, current_field: &mut Field) {
    match std::mem::replace(builder, ActiveTransformBuilder::None) {
        ActiveTransformBuilder::String(t) => current_field.transform.push(Transform::String(t)),
        ActiveTransformBuilder::Number(t) => current_field.transform.push(Transform::Number(t)),
        ActiveTransformBuilder::File(t) => current_field.transform.push(Transform::File(t)),
        ActiveTransformBuilder::Boolean(t) => current_field.transform.push(Transform::Boolean(t)),
        ActiveTransformBuilder::Array(t) => current_field.transform.push(Transform::Array(t)),
        ActiveTransformBuilder::Enum(t) => current_field.transform.push(Transform::Enum(t)),
        // todo[Add]: Type
        ActiveTransformBuilder::None => {}
    }
}

// todo[Add]: Type
enum ActiveRuleBuilder {
    String(StringRules),
    Number(NumberRules),
    File(FileRules),
    Boolean(BooleanRules),
    Array(ArrayRules),
    Enum(EnumRules),
    None,
}

// todo[Add]: Type
enum ActiveTransformBuilder {
    String(StringTransform),
    Number(NumberTransform),
    File(FileTransform),
    Boolean(BooleanTransform),
    Array(ArrayTransform),
    Enum(EnumTransform),
    None,
}

trait BuilderTrait {
    fn get_type(&self) -> Option<FieldType>;
}

impl BuilderTrait for ActiveRuleBuilder {
    /// get the type of this builder
    fn get_type(&self) -> Option<FieldType> {
        // todo[Add]: Type
        match self {
            ActiveRuleBuilder::String(_) => Some(FieldType::String),
            ActiveRuleBuilder::Number(_) => Some(FieldType::Number),
            ActiveRuleBuilder::File(_) => Some(FieldType::File),
            ActiveRuleBuilder::Boolean(_) => Some(FieldType::Boolean),
            ActiveRuleBuilder::Array(_) => Some(FieldType::Array),
            ActiveRuleBuilder::Enum(_) => Some(FieldType::Enum),
            ActiveRuleBuilder::None => None,
        }
    }
}

impl BuilderTrait for ActiveTransformBuilder {
    /// get the type of this builder
    fn get_type(&self) -> Option<FieldType> {
        // todo[Add]: Type
        match self {
            ActiveTransformBuilder::String(_) => Some(FieldType::String),
            ActiveTransformBuilder::Number(_) => Some(FieldType::Number),
            ActiveTransformBuilder::File(_) => Some(FieldType::File),
            ActiveTransformBuilder::Boolean(_) => Some(FieldType::Boolean),
            ActiveTransformBuilder::Array(_) => Some(FieldType::Array),
            ActiveTransformBuilder::Enum(_) => Some(FieldType::Enum),
            ActiveTransformBuilder::None => None,
        }
    }
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
    let mut active_transform_builder = ActiveTransformBuilder::None;
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
                if let Some(form) = forms.get_mut(&current_form_name) {
                    if let Some(field) = form.fields.get_mut(&current_field_name) {
                        flush_rules(&mut active_rule_builder, field);
                        flush_transforms(&mut active_transform_builder, field);
                    }
                }
            }

            if (new_level as usize) <= (Level::Field as usize) {
                if let Some(form) = forms.get_mut(&current_form_name) {
                    if let Some(finished_field) = form.fields.get_mut(&current_field_name) {
                        merge_rules(finished_field, &mut errors);
                        merge_transforms(finished_field, &mut errors);
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

                parsing_type = FieldType::String; // This this temporary value that will be
                                                  // overwritten later
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

                        // todo[Add]: Type
                        let field_type: FieldType = match value {
                            "string" => FieldType::String,
                            "number" => FieldType::Number,
                            "file" => FieldType::File,
                            "boolean" | "bool" => FieldType::Boolean,
                            "array" => FieldType::Array,
                            "enum" => FieldType::Enum,
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
                            FieldType::String => ActiveRuleBuilder::String(StringRules::new()),
                            FieldType::File => ActiveRuleBuilder::File(FileRules::new()),
                            FieldType::Boolean => ActiveRuleBuilder::Boolean(BooleanRules::new()),
                            FieldType::Array => ActiveRuleBuilder::Array(ArrayRules::new()),
                            FieldType::Enum => ActiveRuleBuilder::Enum(EnumRules::new()),
                            // todo[Add]: Type
                            _ => {
                                errors.push(format!("Unknown field type {}", value));
                                continue;
                            }
                        };
                    }

                    // B. PARSE (Variations 1, 2, 3)
                    let (final_val, final_err): (serde_yaml_ng::Value, Option<String>) = {
                        if value.starts_with('{') {
                            // Variation 1: Inline JSON
                            // <key>: { value: <value>, error: <error> }
                            let rt: RuleType<serde_yaml_ng::Value> = serde_yaml_ng::from_str(value)
                                .map_err(|e| vec![format!("Invalid inline rule: {}", e)])?;
                            (rt.value, rt.error)
                        } else if value.is_empty() {
                            // Variation 3: Nested Block (Lookahead)
                            // <key>:
                            //    value: <value>
                            //    error: <error>
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
                            // <key>: <value>
                            // error: <error>
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
                        ActiveRuleBuilder::File(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Boolean(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Array(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Enum(r) => r.set_rule(key, final_val, final_err),
                        // todo[Add]: Type
                        ActiveRuleBuilder::None => Ok(()),
                    };

                    if let Err(msg) = result {
                        errors.push(format!("Rule Error at {}: {}", current_field_name, msg));
                        continue;
                    }
                }
                "transform" => {
                    let current_form = forms.get_mut(&current_form_name).unwrap();
                    let current_field = current_form.fields.get_mut(&current_field_name).unwrap();

                    // INITIALIZE BUILDER
                    if matches!(active_transform_builder, ActiveTransformBuilder::None) {
                        active_transform_builder = match parsing_type {
                            FieldType::Number => {
                                ActiveTransformBuilder::Number(NumberTransform::new())
                            }
                            FieldType::String => {
                                ActiveTransformBuilder::String(StringTransform::new())
                            }
                            FieldType::File => ActiveTransformBuilder::File(FileTransform::new()),
                            FieldType::Boolean => {
                                ActiveTransformBuilder::Boolean(BooleanTransform::new())
                            }
                            FieldType::Array => {
                                ActiveTransformBuilder::Array(ArrayTransform::new())
                            }
                            FieldType::Enum => ActiveTransformBuilder::Enum(EnumTransform::new()),
                            // todo[Add]: Type
                            _ => {
                                errors.push(format!("Unknown field type {}", value));
                                continue;
                            }
                        };
                    }

                    match key {
                        "cast" => {
                            // 1. Parse the target type
                            // todo[Add]: Type
                            let cast_type = match value {
                                "string" => FieldType::String,
                                "number" => FieldType::Number,
                                "boolean" => FieldType::Boolean,
                                "array" => FieldType::Array,
                                "enum" => FieldType::Enum,
                                "file" => FieldType::File,
                                _ => {
                                    errors.push(format!("Invalid cast type '{}'", value));
                                    continue;
                                }
                            };

                            // 2. Update the field type immediately
                            parsing_type = cast_type;

                            // build transformation
                            build_transform(key, value, &mut active_transform_builder, &mut errors);

                            // 3. checking for conflict and flushing
                            if let Some(rule_builder_type) = active_rule_builder.get_type() {
                                if rule_builder_type != parsing_type {
                                    // The builder's type doesn't match the new parsing type.
                                    // Flush rules and reset.
                                    flush_rules(&mut active_rule_builder, current_field);
                                    active_rule_builder = ActiveRuleBuilder::None;
                                }
                            }
                            if let Some(transform_builder_type) =
                                active_transform_builder.get_type()
                            {
                                if transform_builder_type != parsing_type {
                                    // The builder's type doesn't match the new parsing type.
                                    // Flush tansforms and reset.
                                    flush_transforms(&mut active_transform_builder, current_field);
                                    active_transform_builder = ActiveTransformBuilder::None;
                                }
                            }
                        }
                        // String only transforms
                        "split" | "trim" | "to_lowercase" | "to_lower_case" | "toLowerCase"
                        | "lowercase" | "to_uppercase" | "to_upper_case" | "toUpperCase"
                        | "uppercase" => {
                            if current_field.field_type != FieldType::String {
                                errors.push(format!(
                                    "Cannot use the {} transform non-string field {}",
                                    key, current_field_name
                                ));
                            }

                            build_transform(key, value, &mut active_transform_builder, &mut errors);
                        }
                        // end of String only transforms
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
            Level::ValueAndErrorPair => unreachable!(), // This branch is unreachable
        }
        prev_spaces = current_spaces;
    }

    // EOF
    // This happens when we reach the end of the file
    // we flush rules and transforms,
    // and merge possible rules and transforms
    if let Some(form) = forms.get_mut(&current_form_name) {
        if let Some(field) = form.fields.get_mut(&current_field_name) {
            flush_rules(&mut active_rule_builder, field);
            flush_transforms(&mut active_transform_builder, field);
            merge_rules(field, &mut errors);
            merge_transforms(field, &mut errors);
        }
    }

    // collect rules togather
    if errors.is_empty() {
        Ok(forms)
    } else {
        Err(errors)
    }
}

fn build_transform(
    key: &str,
    value: &str,
    builder: &mut ActiveTransformBuilder,
    errors: &mut Vec<String>,
) {
    let clean_val_str = value.trim().trim_end_matches(',');
    let final_val = serde_yaml_ng::from_str(clean_val_str).unwrap();

    // todo[Add]: Type
    let result = match builder {
        ActiveTransformBuilder::String(string_transform) => {
            string_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Number(number_transform) => {
            number_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::File(file_transform) => {
            file_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Boolean(boolean_transform) => {
            boolean_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Array(array_transform) => {
            array_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Enum(enum_transform) => {
            enum_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::None => Ok(()),
    };

    if let Err(r) = result {
        errors.push(format!("Transformation Error: {}", r).to_string());
    }
}

fn merge_rules(field: &mut Field, errors: &mut Vec<String>) {
    // 1. Take the rules out (empties the field's vector)
    let raw_rules = std::mem::take(&mut field.rules);

    // 2. Prepare accumulator
    let mut accumulated_rules: Vec<Rule> = Vec::new();

    // 3. Iterate and Merge
    for rule in raw_rules {
        if let Some(existing) = accumulated_rules.iter_mut().find(|r| r.is_same_type(&rule)) {
            // This merges rules for the same types
            existing.merge(rule, errors);
        } else {
            // This adds the rules for types seen for the first time
            accumulated_rules.push(rule);
        }
    }

    // 4. Push back the merged results
    field.rules = accumulated_rules;
}

fn merge_transforms(field: &mut Field, errors: &mut Vec<String>) {
    // 1. Take the rules out (empties the field's vector)
    let raw_transforms = std::mem::take(&mut field.transform);

    // 2. Prepare accumulator
    let mut accumulated_transforms: Vec<Transform> = Vec::new();

    // 3. Iterate and Merge
    for transform in raw_transforms {
        if let Some(existing) = accumulated_transforms
            .iter_mut()
            .find(|r| r.is_same_type(&transform))
        {
            // This merges rules for the same types
            existing.merge(transform, errors);
        } else {
            // This adds the rules for types seen for the first time
            accumulated_transforms.push(transform);
        }
    }

    // 4. Push back the merged results
    field.transform = accumulated_transforms;
}
