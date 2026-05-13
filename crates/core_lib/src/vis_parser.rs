use crate::ast::{
    ArrayRules, ArrayTransform, Base64Rules, Base64Transform, BooleanRules, BooleanTransform, Field,
    FieldType, FileRules, FileTransform, Form, ImageRules, ImageTransform, MailRules,
    MailTransform, NumberRules, NumberTransform, Rule, RuleTrait, RuleType, StringRules,
    StringTransform, Transform, TransformTrait, UsernameRules, UsernameTransform, UuidRules,
    UuidTransform,
};
use indexmap::IndexMap;
use std::collections::HashMap;

// todo[Add]: Type
enum ActiveRuleBuilder {
    String(StringRules),
    Number(NumberRules),
    File(FileRules),
    Boolean(BooleanRules),
    Array(ArrayRules),
    Enum(EnumRules),
    Image(ImageRules),
    Mail(MailRules),
    Username(UsernameRules),
    Uuid(UuidRules),
    Base64(Base64Rules),
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
    Image(ImageTransform),
    Mail(MailTransform),
    Username(UsernameTransform),
    Uuid(UuidTransform),
    Base64(Base64Transform),
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
            ActiveRuleBuilder::Image(_) => Some(FieldType::Image),
            ActiveRuleBuilder::Mail(_) => Some(FieldType::Mail),
            ActiveRuleBuilder::Username(_) => Some(FieldType::Username),
            ActiveRuleBuilder::Uuid(_) => Some(FieldType::Uuid),
            ActiveRuleBuilder::Base64(_) => Some(FieldType::Base64),
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
            ActiveTransformBuilder::Image(_) => Some(FieldType::Image),
            ActiveTransformBuilder::Mail(_) => Some(FieldType::Mail),
            ActiveTransformBuilder::Username(_) => Some(FieldType::Username),
            ActiveTransformBuilder::Uuid(_) => Some(FieldType::Uuid),
            ActiveTransformBuilder::Base64(_) => Some(FieldType::Base64),
            ActiveTransformBuilder::None => None,
        }
    }
}

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
        ActiveRuleBuilder::Image(r) => current_field.rules.push(Rule::Image(r)),
        ActiveRuleBuilder::Mail(r) => current_field.rules.push(Rule::Mail(r)),
        ActiveRuleBuilder::Username(r) => current_field.rules.push(Rule::Username(r)),
        ActiveRuleBuilder::Uuid(r) => current_field.rules.push(Rule::Uuid(r)),
        ActiveRuleBuilder::Base64(r) => current_field.rules.push(Rule::Base64(r)),
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
        ActiveTransformBuilder::Image(t) => current_field.transform.push(Transform::Image(t)),
        ActiveTransformBuilder::Mail(t) => current_field.transform.push(Transform::Mail(t)),
        ActiveTransformBuilder::Username(t) => current_field.transform.push(Transform::Username(t)),
        ActiveTransformBuilder::Uuid(t) => current_field.transform.push(Transform::Uuid(t)),
        ActiveTransformBuilder::Base64(t) => current_field.transform.push(Transform::Base64(t)),
        // todo[Add]: Type
        ActiveTransformBuilder::None => {}
    }
}

// 1. Define the Level Enum
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Level {
    Form,
    Field,
    Property,
    RulesAndTransformations,
}

// Helper to convert integer depth to Enum
impl Level {
    fn from_depth(depth: usize) -> Result<Level, String> {
        match depth {
            0 => Ok(Level::Form),
            1 => Ok(Level::Field),
            2 => Ok(Level::Property),
            3 => Ok(Level::RulesAndTransformations),
            n => Err(format!("Nesting too deep: level {}", n)),
        }
    }

    fn get_next_level(&self) -> Result<Level, String> {
        match *self {
            Level::Form => Ok(Level::Field),
            Level::Field => Ok(Level::Property),
            Level::Property => Ok(Level::RulesAndTransformations),
            Level::RulesAndTransformations => Err("Error: Nesting too deep".to_string()),
        }
    }

    fn get_level_from_index(i: usize) -> Result<Level, String> {
        match i {
            0 => Ok(Level::Form),
            1 => Ok(Level::Field),
            2 => Ok(Level::Property),
            3 => Ok(Level::RulesAndTransformations),
            _ => Err("Invalid level Index".to_string()),
        }
    }
}

/// helper function to get the number of spaces at the start of a line
pub fn raw_spaces(line: &str) -> Result<usize, &'static str> {
    let mut count = 0;
    let mut seen_space = false;
    let mut seen_tab = false;

    for c in line.chars() {
        if c == ' ' {
            seen_space = true;
            count += 1;
        } else if c == '\t' {
            seen_tab = true;
            count += 1;
        } else {
            break; // End of indentation whitespace
        }

        // The trap: If both flags ever become true, they mixed them!
        if seen_space && seen_tab {
            return Err("Inconsistent indentation: Cannot mix tabs and spaces on the same line.");
        }
    }

    Ok(count)
}

// Parser errors type
#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: u32,
    pub start_col: u32,
    pub end_col: u32,
}

impl ParserError {
    fn new(message: String, line: u32, start_col: u32, end_col: u32) -> ParserError {
        ParserError {
            message,
            line,
            start_col,
            end_col,
        }
    }
}

pub fn parse_vis(input: &str) -> Result<IndexMap<String, Form>, Vec<ParserError>> {
    let mut forms: IndexMap<String, Form> = IndexMap::new();
    let mut errors: Vec<ParserError> = Vec::new();

    let mut current_level = Level::Form;
    let mut levels_vector = vec![0; 4]; // this vector stores the level depth and is indexed

    let mut parsing_type = FieldType::String;
    // through the Level enum
    let mut prev_spaces = 0;
    let lines = input
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
        .enumerate();

    // used for final erroring
    let number_of_lines = lines.clone().count();
    let last_line_length = lines.clone().last().iter().len();

    let mut iter = lines.into_iter().peekable();

    let mut current_form_name = String::new();
    let mut current_field_name = String::new();
    let mut active_context = "none"; // transform or rule
    let mut active_rule_builder = ActiveRuleBuilder::None;
    let mut active_transform_builder = ActiveTransformBuilder::None;
    let mut prev_level = Level::Form;
    let mut is_type_defined = false; // This variable is used to know if the type has been defined

    while let Some((line_index, line)) = iter.next() {
        let current_spaces = match raw_spaces(line) {
            Ok(spaces) => spaces,
            Err(msg) => {
                errors.push(ParserError::new(
                    msg.to_string(),
                    line_index as u32,
                    current_level as u32, // Or a generic column 0
                    current_level as u32 + line.len() as u32,
                ));
                continue;
            }
        };
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
        let key = parts[0].trim();
        let value = if parts.len() > 1 { parts[1].trim() } else { "" };

        if current_spaces > prev_spaces {
            current_level = current_level.get_next_level().unwrap_or_else(|err| {
                errors.push(ParserError::new(
                    err,
                    line_index as u32,
                    current_level as u32,
                    current_level as u32 + line.len() as u32,
                ));

                Level::RulesAndTransformations
            });

            levels_vector[current_level as usize] = current_spaces;
        } else if current_spaces < prev_spaces {
            let new_level_idx = levels_vector
                .iter()
                .position(|&v| v == current_spaces)
                .unwrap_or_else(|| {
                    errors.push(ParserError::new(
                        String::from("Invalid Nesting"),
                        line_index as u32,
                        current_level as u32,
                        current_level as u32 + line.len() as u32,
                    ));

                    Level::RulesAndTransformations as usize
                });
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

            // new field
            if (new_level as usize) <= (Level::Field as usize) {
                is_type_defined = false; // resets the type defined flag
                if let Some(form) = forms.get_mut(&current_form_name) {
                    if let Some(finished_field) = form.fields.get_mut(&current_field_name) {
                        merge_rules(finished_field).unwrap_or_else(|error| {
                            errors.push(ParserError::new(
                                error,
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
                        });
                        merge_transforms(finished_field).unwrap_or_else(|error| {
                            errors.push(ParserError::new(
                                error,
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
                        });
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
                    errors.push(ParserError::new(
                        format!("Error: Form {} cannot be set to a single value", key),
                        line_index as u32,
                        current_level as u32,
                        current_level as u32 + line.len() as u32,
                    ));
                    continue;
                }

                if forms.contains_key(key) {
                    errors.push(ParserError::new(
                        format!("Duplicate Form name: {}", key),
                        line_index as u32,
                        current_level as u32,
                        current_level as u32 + line.len() as u32,
                    ));
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

                // check that there is no value after the field name like `username: value`
                if !value.is_empty() {
                    errors.push(ParserError::new(
                        format!("Error: Field {} cannot be set to a single value", key),
                        line_index as u32,
                        current_level as u32,
                        current_level as u32 + line.len() as u32,
                    ));
                    continue;
                }

                // check for duplicate fields
                if current_form.fields.contains_key(key) {
                    errors.push(ParserError::new(
                        format!("Duplicate Field name: {}", key),
                        line_index as u32,
                        current_level as u32,
                        current_level as u32 + line.len() as u32,
                    ));
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
                let current_field = current_form
                    .fields
                    .get_mut(&current_field_name)
                    //.unwrap()
                    .expect(&format!(
                        "crashed at line: {}\nNext line: {}",
                        line,
                        iter.clone().peekable().peek().unwrap().1,
                    ));

                match property_name {
                    // match all possible properties
                    "type" | "fieldType" => {
                        if value.is_empty() {
                            errors.push(ParserError::new(
                                format!("Error: field type cannot be empty"),
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
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
                            "image" => FieldType::Image,
                            "mail" => FieldType::Mail,
                            "username" => FieldType::Username,
                            "uuid" => FieldType::Uuid,
                            _ => {
                                errors.push(ParserError::new(
                                    format!("Unknown field type {}", value),
                                    line_index as u32,
                                    current_level as u32,
                                    current_level as u32 + line.len() as u32,
                                ));
                                continue;
                            }
                        };

                        current_field.field_type = field_type;
                        parsing_type = field_type;
                        is_type_defined = true;
                    }
                    "required" => {
                        if value.is_empty() {
                            errors.push(ParserError::new(
                                format!("Error: required cannot be empty"),
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
                            continue;
                        }

                        match value {
                            "true" => current_field.required = true,
                            "false" => current_field.required = false,
                            _ => {
                                errors.push(ParserError::new(
                                    format!("Unknown required value {}", value),
                                    line_index as u32,
                                    current_level as u32,
                                    current_level as u32 + line.len() as u32,
                                ));
                                continue;
                            }
                        };
                    }
                    "defaultError" => {
                        current_field.default_error = Some(value.to_string());
                    }
                    "rules" => {
                        if !value.is_empty() {
                            errors.push(ParserError::new(
                                format!("Error: rules cannot be set to a single value"),
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
                            continue;
                        }

                        active_context = "rules";
                    }
                    "transform" => {
                        if !value.is_empty() {
                            errors.push(ParserError::new(
                                format!("Error: transforms cannot be set to a single value"),
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
                            continue;
                        }

                        active_context = "transform";
                    }
                    _ => {
                        errors.push(ParserError::new(
                            format!(
                                "Unknown property {} at {}",
                                property_name, &current_field_name
                            ),
                            line_index as u32,
                            current_level as u32,
                            current_level as u32 + line.len() as u32,
                        ));
                    }
                }
            }
            Level::RulesAndTransformations => match active_context {
                "rules" => {
                    let current_form = forms.get_mut(&current_form_name).unwrap();
                    let current_field = current_form.fields.get_mut(&current_field_name).unwrap();

                    // check if the type has already been defined or not
                    if !is_type_defined {
                        errors.push(ParserError::new(
                            format!("Error: Cannot set rules before defining a type"),
                            line_index as u32,
                            current_level as u32,
                            current_level as u32 + line.len() as u32,
                        ));
                        continue;
                    }

                    // A. INITIALIZE BUILDER
                    if matches!(active_rule_builder, ActiveRuleBuilder::None) {
                        active_rule_builder = match parsing_type {
                            FieldType::Number => ActiveRuleBuilder::Number(NumberRules::new()),
                            FieldType::String => ActiveRuleBuilder::String(StringRules::new()),
                            FieldType::File => ActiveRuleBuilder::File(FileRules::new()),
                            FieldType::Boolean => ActiveRuleBuilder::Boolean(BooleanRules::new()),
                            FieldType::Array => ActiveRuleBuilder::Array(ArrayRules::new()),
                            FieldType::Enum => ActiveRuleBuilder::Enum(EnumRules::new()),
                            FieldType::Image => ActiveRuleBuilder::Image(ImageRules::new()),
                            FieldType::Mail => ActiveRuleBuilder::Mail(MailRules::new()),
                            FieldType::Username => {
                                ActiveRuleBuilder::Username(UsernameRules::new())
                            }
                            FieldType::Uuid => ActiveRuleBuilder::Uuid(UuidRules::new()),
                            // todo[Add]: Type
                            _ => {
                                errors.push(ParserError::new(
                                    format!("Unknown field type {:?}", parsing_type),
                                    line_index as u32,
                                    current_level as u32,
                                    current_level as u32 + line.len() as u32,
                                ));
                                continue;
                            }
                        };
                    }

                    // B. PARSE (Variations 1, 2, 3)
                    let (final_val, final_err): (serde_yaml_ng::Value, Option<String>) = {
                        let trimmed_val = value.trim();

                        if trimmed_val.starts_with('{') {
                            // VARIATION 1: Inline JSON
                            let rt_result: Result<RuleType<serde_yaml_ng::Value>, _> =
                                serde_yaml_ng::from_str(trimmed_val);
                            match rt_result {
                                Ok(rt) => (rt.value, rt.error),
                                Err(e) => {
                                    errors.push(ParserError::new(
                                        format!("Invalid inline rule: {}", e),
                                        line_index as u32,
                                        current_level as u32,
                                        current_level as u32 + line.len() as u32,
                                    ));
                                    continue;
                                }
                            }
                        } else if trimmed_val.is_empty() {
                            // VARIATION 3: Nested Block (Borrow-Safe)
                            let mut n_val = serde_yaml_ng::Value::Null;
                            let mut n_err = None;
                            let mut child_level_spaces = 0;

                            loop {
                                // Scope the peek so the borrow is dropped immediately
                                let peek_info = if let Some(&(peek_index, peek_line)) = iter.peek()
                                {
                                    match raw_spaces(peek_line) {
                                        Ok(spaces) => Some((peek_index, spaces)),
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let Some((peek_index, p_spaces)) = peek_info else {
                                    break;
                                };

                                if p_spaces <= current_spaces {
                                    break;
                                }

                                if child_level_spaces != 0 && child_level_spaces != p_spaces {
                                    errors.push(ParserError::new(
                                        String::from("Invalid Nesting"),
                                        peek_index as u32,
                                        current_level as u32,
                                        current_level as u32 + p_spaces as u32,
                                    ));
                                }
                                child_level_spaces = p_spaces;

                                // Safe to consume now
                                let (_, child_line) = iter.next().unwrap();
                                let c_parts: Vec<&str> = child_line.splitn(2, ':').collect();
                                let c_key = c_parts[0].trim();
                                let c_val_clean = if c_parts.len() > 1 { c_parts[1] } else { "" }
                                    .split('#')
                                    .next()
                                    .unwrap_or("")
                                    .trim();

                                match c_key {
                                    "value" => {
                                        n_val = serde_yaml_ng::from_str(c_val_clean).unwrap_or_else(
                                            |_| {
                                                serde_yaml_ng::Value::String(
                                                    c_val_clean.to_string(),
                                                )
                                            },
                                        )
                                    }
                                    "error" => n_err = Some(c_val_clean.replace(['\'', '"'], "")),
                                    _ => errors.push(ParserError::new(
                                        format!("Unknown nested key '{}'", c_key),
                                        peek_index as u32,
                                        current_level as u32,
                                        current_level as u32 + child_line.len() as u32,
                                    )),
                                }
                            }
                            (n_val, n_err)
                        } else {
                            // VARIATION 2: Sibling Error (Borrow-Safe)
                            let clean_val_str = trimmed_val
                                .trim_end_matches(',')
                                .split('#')
                                .next()
                                .unwrap_or("")
                                .trim();
                            let s_val =
                                serde_yaml_ng::from_str(clean_val_str).unwrap_or_else(|_| {
                                    serde_yaml_ng::Value::String(clean_val_str.to_string())
                                });

                            let mut s_err = None;
                            let mut consume_error = false;
                            let mut extracted_err = None;

                            // Scope the peek
                            if let Some(&(_, peek_line)) = iter.peek() {
                                if let Ok(p_spaces) = raw_spaces(peek_line) {
                                    if p_spaces == current_spaces {
                                        let p_parts: Vec<&str> = peek_line.splitn(2, ':').collect();
                                        if p_parts[0].trim() == "error" {
                                            consume_error = true;
                                            let raw_err =
                                                if p_parts.len() > 1 { p_parts[1] } else { "" };
                                            extracted_err = Some(
                                                raw_err
                                                    .split('#')
                                                    .next()
                                                    .unwrap_or("")
                                                    .trim()
                                                    .to_string(),
                                            );
                                        }
                                    }
                                }
                            }

                            // Consume outside the peek scope
                            if consume_error {
                                iter.next();
                                if let Some(err_str) = extracted_err {
                                    s_err = Some(err_str.replace(['\'', '"'], ""));
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
                        ActiveRuleBuilder::Image(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Mail(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Username(r) => r.set_rule(key, final_val, final_err),
                        ActiveRuleBuilder::Uuid(r) => r.set_rule(key, final_val, final_err),
                        // todo[Add]: Type
                        ActiveRuleBuilder::None => Ok(()),
                    };

                    if let Err(msg) = result {
                        errors.push(ParserError::new(
                            format!("Rule Error at {}: {}", current_field_name, msg),
                            line_index as u32,
                            current_level as u32,
                            current_level as u32 + line.len() as u32,
                        ));
                        continue;
                    }
                }
                "transform" => {
                    let current_form = forms.get_mut(&current_form_name).unwrap();
                    let current_field = current_form.fields.get_mut(&current_field_name).unwrap();

                    if current_field.rules.len() < current_field.transform.len() + 1 {
                        let placeholder_rule = match parsing_type {
                            FieldType::String => Rule::String(StringRules::new()),
                            FieldType::Number => Rule::Number(NumberRules::new()),
                            FieldType::Boolean => Rule::Boolean(BooleanRules::new()),
                            FieldType::Array => Rule::Array(ArrayRules::new()),
                            FieldType::File => Rule::File(FileRules::new()),
                            FieldType::Enum => Rule::Enum(EnumRules::new()),
                            // todo[Add]: type
                            FieldType::Image => Rule::Image(ImageRules::new()),
                            FieldType::Mail => Rule::Mail(MailRules::new()),
                            FieldType::Username => Rule::Username(UsernameRules::new()),
                            FieldType::Uuid => Rule::Uuid(UuidRules::new()),
                            FieldType::Password => todo!(),
                            FieldType::Url => todo!(),
                            FieldType::HttpUrl => todo!(),
                            FieldType::Base64 => todo!(),
                            FieldType::Jwt => todo!(),
                            FieldType::Hex => todo!(),
                            FieldType::Cidrv4 => todo!(),
                            FieldType::Cidrv6 => todo!(),
                            FieldType::Ulid => todo!(),
                            FieldType::Cuid2 => todo!(),
                            FieldType::Hash => todo!(),
                            FieldType::Date => todo!(),
                        };

                        current_field.rules.push(placeholder_rule);
                    }

                    // check if the type has already been defined or not
                    if !is_type_defined {
                        errors.push(ParserError::new(
                            format!("Error: Cannot set transforms before defining a type"),
                            line_index as u32,
                            current_level as u32,
                            current_level as u32 + line.len() as u32,
                        ));
                        continue;
                    }

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
                            FieldType::Image => ActiveTransformBuilder::Image(ImageTransform::new()),
                            FieldType::Mail => ActiveTransformBuilder::Mail(MailTransform::new()),
                            FieldType::Username => {
                                ActiveTransformBuilder::Username(UsernameTransform::new())
                            }
                            FieldType::Uuid => ActiveTransformBuilder::Uuid(UuidTransform::new()),
                            // todo[Add]: Type
                            _ => {
                                errors.push(ParserError::new(
                                    format!("Unknown field type {}", value),
                                    line_index as u32,
                                    current_level as u32,
                                    current_level as u32 + line.len() as u32,
                                ));
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
                                    errors.push(ParserError::new(
                                        format!("Invalid cast type '{}'", value),
                                        line_index as u32,
                                        current_level as u32,
                                        current_level as u32 + line.len() as u32,
                                    ));
                                    continue;
                                }
                            };

                            // 2. Update the field type immediately
                            parsing_type = cast_type;

                            // build transformation
                            build_transform(
                                key,
                                value,
                                &mut active_transform_builder,
                                &mut errors,
                                line_index,
                                current_level,
                                line,
                            );

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
                            if current_field.field_type != FieldType::String
                                && current_field.field_type != FieldType::Username
                                && current_field.field_type != FieldType::Uuid
                            {
                                errors.push(ParserError::new(
                                    format!(
                                        "Cannot use the {} transform non-string field {}",
                                        key, current_field_name
                                    ),
                                    line_index as u32,
                                    current_level as u32,
                                    current_level as u32 + line.len() as u32,
                                ));
                            }

                            if key == "split" {
                                parsing_type = FieldType::Array;
                            }

                            build_transform(
                                key,
                                value,
                                &mut active_transform_builder,
                                &mut errors,
                                line_index,
                                current_level,
                                line,
                            );
                        }
                        // end of String only transforms
                        // start of array only transforms
                        "join" | "sum" => {
                            if current_field.field_type != FieldType::Array {
                                errors.push(ParserError::new(
                                    format!(
                                        "Cannot use the {} transform non-array field {}",
                                        key, current_field_name
                                    ),
                                    line_index as u32,
                                    current_level as u32,
                                    current_level as u32 + line.len() as u32,
                                ));
                            }

                            if key == "join" {
                                parsing_type = FieldType::String;
                            }

                            build_transform(
                                key,
                                value,
                                &mut active_transform_builder,
                                &mut errors,
                                line_index,
                                current_level,
                                line,
                            );
                        }
                        _ => {
                            errors.push(ParserError::new(
                                format!("Unknown transform property: {}", key),
                                line_index as u32,
                                current_level as u32,
                                current_level as u32 + line.len() as u32,
                            ));
                        }
                    }
                }
                _ => {
                    errors.push(ParserError::new(
                        format!(
                            "Unknown context {} at {}",
                            active_context, &current_field_name
                        ),
                        line_index as u32,
                        current_level as u32,
                        current_level as u32 + line.len() as u32,
                    ));
                }
            },
            _ => {
                errors.push(ParserError::new(
                    String::from("Nesting too deep"),
                    line_index as u32,
                    current_level as u32,
                    current_level as u32 + line.len() as u32,
                ));
                continue;
            }
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
            merge_rules(field).unwrap_or_else(|error| {
                errors.push(ParserError::new(
                    error,
                    number_of_lines as u32,
                    current_level as u32,
                    current_level as u32 + last_line_length as u32,
                ));
            });
            merge_transforms(field).unwrap_or_else(|error| {
                errors.push(ParserError::new(
                    error,
                    number_of_lines as u32,
                    current_level as u32,
                    current_level as u32 + last_line_length as u32,
                ));
            });
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
    errors: &mut Vec<ParserError>,
    line_index: usize,
    current_level: Level,
    line: &str,
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
        ActiveTransformBuilder::Image(image_transform) => {
            image_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Mail(mail_transform) => {
            mail_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Username(username_transform) => {
            username_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::Uuid(uuid_transform) => {
            uuid_transform.set_transform(key, final_val)
        }
        ActiveTransformBuilder::None => Ok(()),
    };

    if let Err(r) = result {
        errors.push(ParserError::new(
            format!("Transformation Error: {}", r),
            line_index as u32,
            current_level as u32,
            current_level as u32 + line.len() as u32,
        ));
    }
}

fn merge_rules(field: &mut Field) -> Result<(), String> {
    // 1. Take the rules out (empties the field's vector)
    let raw_rules = std::mem::take(&mut field.rules);

    // 2. Prepare accumulator
    let mut accumulated_rules: Vec<Rule> = Vec::new();

    // 3. Iterate and Merge
    for rule in raw_rules {
        if let Some(existing) = accumulated_rules.iter_mut().find(|r| r.is_same_type(&rule)) {
            // This merges rules for the same types
            match existing.merge(rule) {
                Err(e) => return Err(e),
                Ok(_) => (),
            }
        } else {
            // This adds the rules for types seen for the first time
            accumulated_rules.push(rule);
        }
    }

    // 4. Push back the merged results
    field.rules = accumulated_rules;
    Ok(())
}

fn merge_transforms(field: &mut Field) -> Result<(), String> {
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
            match existing.merge(transform) {
                Err(e) => return Err(e),
                Ok(_) => (),
            }
        } else {
            // This adds the rules for types seen for the first time
            accumulated_transforms.push(transform);
        }
    }

    // 4. Push back the merged results
    field.transform = accumulated_transforms;
    Ok(())
}
