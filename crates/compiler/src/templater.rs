use core_lib::ast::{Field, FieldType, Form, Rule, Transform};
use indexmap::IndexMap;
use rust_embed::RustEmbed;
use serde_json::{Map, Value};
use tera::{Context, Tera};

use crate::config::{self, Language, LanguageConfig};

#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Templates;

// first String for file name, second String for form name
pub fn templater(files: IndexMap<String, IndexMap<String, Form>>, config: config::Config) {
    for entry in config.languages.iter() {
        let (language, language_config): (&Language, &LanguageConfig) = entry;
        if !language_config.enabled {
            continue;
        }

        println!("Templating for language: {:?}", language);
        if let Some(templater) = LanguageTemplater::from_language(language) {
            templater.template(&files, &language_config.output);
        } else {
            eprintln!("Unsupported language: {:?}", language);
        }
    }
}

struct LanguageTemplater {
    language: String,
    extension: String,
    tera: Tera,
}

impl LanguageTemplater {
    fn from_language(language: &Language) -> Option<Self> {
        let (lang_name, extension) = match language {
            Language::Javascript => ("javascript", "js"),
            Language::CSharp => ("csharp", "cs"),
        };

        let mut tera = Tera::default();
        let prefix = format!("{}/", lang_name);

        let mut templates = Vec::new();
        for file_path in Templates::iter() {
            if file_path.starts_with(&prefix) && file_path.ends_with(".tera") {
                if let Some(embedded_file) = Templates::get(&file_path) {
                    match std::str::from_utf8(embedded_file.data.as_ref()) {
                        Ok(content) => {
                            let template_name = file_path.trim_start_matches(&prefix).to_string();
                            templates.push((template_name, content.to_string()));
                        }
                        Err(e) => {
                            eprintln!("Error reading embedded template '{}': {}", file_path, e);
                        }
                    }
                }
            }
        }

        if let Err(e) = tera.add_raw_templates(templates) {
            eprintln!("Error initializing Tera templates for {}: {}", lang_name, e);
            return None;
        }

        Some(Self {
            language: lang_name.to_string(),
            extension: extension.to_string(),
            tera,
        })
    }

    fn template(&self, files: &IndexMap<String, IndexMap<String, Form>>, output_dir: &String) {
        if !std::path::Path::new(output_dir).exists() {
            std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
        }

        for (file_name, forms) in files.iter() {
            let uses_file_type = has_file_type(forms);
            let uses_uuid_type = has_uuid_type(forms);

            let mut context = Context::new();
            context.insert("actions", &build_actions(forms));
            context.insert("uses_file_type", &uses_file_type);
            context.insert("uses_uuid_type", &uses_uuid_type);

            let output = match self.tera.render("base.tera", &context) {
                Ok(rendered) => rendered,
                Err(error) => {
                    eprintln!(
                        "Error rendering base template for language '{}', file '{}': {:?}",
                        self.language, file_name, error
                    );
                    continue;
                }
            };

            let output_path = format!("{}/{}.{}", output_dir, file_name, self.extension);
            if let Err(error) = std::fs::write(&output_path, output) {
                eprintln!("Failed to write output file '{}': {}", output_path, error);
                continue;
            }

            if uses_file_type {
                self.write_file_signature_helper(output_dir);
            }

            if uses_uuid_type {
                self.write_uuid_helper(output_dir);
            }
        }
    }

    fn write_file_signature_helper(&self, output_dir: &str) {
        let helper_output = match self.tera.render("utils/file_signature.tera", &Context::new()) {
            Ok(rendered) => rendered,
            Err(error) => {
                eprintln!(
                    "Error rendering helper template for language '{}': {}",
                    self.language, error
                );
                return;
            }
        };

        let helper_dir = format!("{}/utils", output_dir);
        if let Err(error) = std::fs::create_dir_all(&helper_dir) {
            eprintln!(
                "Failed to create helper directory '{}': {}",
                helper_dir, error
            );
            return;
        }

        let helper_file = format!("{}/file_signature.{}", helper_dir, self.extension);
        if let Err(error) = std::fs::write(&helper_file, helper_output) {
            eprintln!("Failed to write helper file '{}': {}", helper_file, error);
        }
    }

    fn write_uuid_helper(&self, output_dir: &str) {
        let helper_output = match self.tera.render("utils/uuid_validator.tera", &Context::new()) {
            Ok(rendered) => rendered,
            Err(error) => {
                eprintln!(
                    "Error rendering UUID helper template for language '{}': {}",
                    self.language, error
                );
                return;
            }
        };

        let helper_dir = format!("{}/utils", output_dir);
        if let Err(error) = std::fs::create_dir_all(&helper_dir) {
            eprintln!(
                "Failed to create helper directory '{}': {}",
                helper_dir, error
            );
            return;
        }

        let helper_file = format!("{}/uuid_validator.{}", helper_dir, self.extension);
        if let Err(error) = std::fs::write(&helper_file, helper_output) {
            eprintln!("Failed to write UUID helper file '{}': {}", helper_file, error);
        }
    }
}

fn build_actions(forms: &IndexMap<String, Form>) -> Value {
    let mut actions = Map::new();

    for (action_name, form) in forms.iter() {
        let mut action_fields = Map::new();

        for (field_name, field) in form.fields.iter() {
            action_fields.insert(field_name.clone(), build_field(field));
        }

        actions.insert(action_name.clone(), Value::Object(action_fields));
    }

    Value::Object(actions)
}

fn build_field(field: &Field) -> Value {
    let mut field_obj = Map::new();

    field_obj.insert(
        "type".to_string(),
        Value::String(field.field_type.as_str().to_string()),
    );
    field_obj.insert("required".to_string(), Value::Bool(field.required));

    if let Some(default_error) = &field.default_error {
        field_obj.insert(
            "defaultError".to_string(),
            Value::String(default_error.clone()),
        );
    }

    let merged_rules = merge_rules(&field.rules);
    if !merged_rules.is_null() {
        field_obj.insert("rules".to_string(), merged_rules);
    }

    let merged_transform = merge_transforms(&field.transform);
    if !merged_transform.is_null() {
        field_obj.insert("transform".to_string(), merged_transform);
    }

    Value::Object(field_obj)
}

fn merge_rules(rules: &[Rule]) -> Value {
    let mut merged = Map::new();

    for rule in rules {
        if let Value::Object(rule_obj) = rule_to_value(rule) {
            for (key, value) in rule_obj {
                merged.insert(key, value);
            }
        }
    }

    if merged.is_empty() {
        Value::Null
    } else {
        Value::Object(merged)
    }
}

fn merge_transforms(transforms: &[Transform]) -> Value {
    let mut merged = Map::new();

    for transform in transforms {
        if let Value::Object(transform_obj) = transform_to_value(transform) {
            for (key, value) in transform_obj {
                merged.insert(key, value);
            }
        }
    }

    if merged.is_empty() {
        Value::Null
    } else {
        Value::Object(merged)
    }
}

fn rule_to_value(rule: &Rule) -> Value {
    let mut value = match rule {
        Rule::String(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Number(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Boolean(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Array(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::File(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Enum(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Image(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Mail(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Username(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Uuid(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Base64(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
        Rule::Hash(rules) => serde_json::to_value(rules).unwrap_or(Value::Null),
    };

    prune_nulls(&mut value);
    value
}

fn transform_to_value(transform: &Transform) -> Value {
    let mut value = match transform {
        Transform::String(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Number(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Boolean(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Array(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::File(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Enum(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Image(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Mail(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Username(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Uuid(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Base64(t) => serde_json::to_value(t).unwrap_or(Value::Null),
        Transform::Hash(t) => serde_json::to_value(t).unwrap_or(Value::Null),
    };

    prune_nulls(&mut value);
    value
}

fn prune_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| {
                prune_nulls(v);
                !v.is_null()
            });
        }
        Value::Array(items) => {
            items.iter_mut().for_each(prune_nulls);
        }
        _ => {}
    }
}

fn has_file_type(forms: &IndexMap<String, Form>) -> bool {
    forms.values().any(|form| {
        form.fields.values().any(|field| {
            field.field_type == FieldType::File
                || field.field_type == FieldType::Image
                || field.rules.iter().any(rule_uses_file_type)
                || field.transform.iter().any(transform_uses_file_type)
        })
    })
}

fn rule_uses_file_type(rule: &Rule) -> bool {
    match rule {
        Rule::File(_) => true,
        Rule::Image(_) => true,
        Rule::Array(rules) => rules.array_type.value == FieldType::File || rules.array_type.value == FieldType::Image,
        _ => false,
    }
}

fn transform_uses_file_type(transform: &Transform) -> bool {
    matches!(transform, Transform::File(_) | Transform::Image(_))
        || transform.get_cast() == Some(FieldType::File)
        || transform.get_cast() == Some(FieldType::Image)
}

fn has_uuid_type(forms: &IndexMap<String, Form>) -> bool {
    forms.values().any(|form| {
        form.fields.values().any(|field| {
            field.field_type == FieldType::Uuid
                || field.rules.iter().any(rule_uses_uuid_type)
                || field.transform.iter().any(transform_uses_uuid_type)
        })
    })
}

fn rule_uses_uuid_type(rule: &Rule) -> bool {
    match rule {
        Rule::Uuid(_) => true,
        Rule::Array(rules) => rules.array_type.value == FieldType::Uuid,
        _ => false,
    }
}

fn transform_uses_uuid_type(transform: &Transform) -> bool {
    matches!(transform, Transform::Uuid(_)) || transform.get_cast() == Some(FieldType::Uuid)
}
