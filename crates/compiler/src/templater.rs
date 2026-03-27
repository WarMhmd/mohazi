use core::panic;
use std::any::{self, Any};

use core_lib::ast::{Field, FieldType, Form, Rule, StringRules, StringTransform, Transform};
use indexmap::{map::Entry, IndexMap};
use tera::{Context, Tera};
use tracing::info;

use crate::config::{self, Language, LanguageConfig};

// first String for file name, second String for form name
pub fn templater(files: IndexMap<String, IndexMap<String, Form>>, config: config::Config) {
    for entry in config.languages.iter() {
        let (language, language_config): (&Language, &LanguageConfig) = entry;
        if !language_config.enabled {
            continue;
        }
        println!("Templating for language: {:?}", language);
        match language {
            Language::Javascript => {
                let tera = match Tera::new("./crates/compiler/src/templates/javascript/*.tera") {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Error parsing templates: {}", e);
                        return;
                    }
                };
                JavascriptTemplater::new().template(&files, tera, &language_config.output);
            }
            _ => {
                eprintln!("Unsupported language: {:?}", language);
            }
        }
    }
}

enum TransformRule {
    Rule(Rule),
    Transform(Transform),
}
trait Templater {
    fn new() -> Self;
    fn template(
        &self,
        files: &IndexMap<String, IndexMap<String, Form>>,
        tera: Tera,
        output_dir: &String,
    ) {
        // check if output_dir exists, if not create it
        if !std::path::Path::new(output_dir).exists() {
            std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
        }

        for (file_name, forms) in files.iter() {
            for (form_name, form) in forms.iter() {
                for entry in form.fields.iter() {
                    let (field_name, field): (&String, &Field) = entry;
                    let path = format!("{}.{}", form_name, field_name);
                    // go_though_list will have one rule then one transform and so on
                    let transform_rules_combined: Vec<TransformRule> = field
                        .rules
                        .iter()
                        .map(|r| TransformRule::Rule(r.clone()))
                        .chain(
                            field
                                .transform
                                .iter()
                                .map(|t| TransformRule::Transform(t.clone())),
                        )
                        .collect();
                    let mut iter = transform_rules_combined.iter().peekable();

                    let mut final_result = String::new();

                    while let Some(rule) = iter.next() {
                        let rule = match rule {
                            TransformRule::Rule(r) => r,
                            _ => panic!("Expected rule but found transform"),
                        };

                        let transform = match iter.peek() {
                            Some(TransformRule::Transform(t)) => {
                                iter.next();
                                Some(t)
                            }
                            _ => None,
                        };

                        // check if template exists for this language and rule
                        if self.check_template_exits(
                            &self.get_language(),
                            &self.get_validate_type(&rule),
                        ) {
                            let context = self.get_context(
                                field_name,
                                &path,
                                field.required,
                                &field
                                    .default_error
                                    .as_ref()
                                    .unwrap_or(&"Invalid value".to_string()),
                                &rule,
                                transform,
                            );
                            info!("transform: {:?}", transform);
                            let result = tera.render(
                                format!("{}.tera", self.get_validate_type(&rule)).as_str(),
                                &context,
                            );

                            if result.is_err() {
                                eprintln!("Error rendering template for language: {}, validate type: {}, error: {:?}", self
									.get_language(), self.get_validate_type(&rule), result.err().unwrap());
                                break;
                            }
                            let result = result.unwrap();
                            if !final_result.is_empty() {
                                final_result.push_str("\n");
                                final_result.push_str("\n");
                            }
                            final_result.push_str(&result);
                            // info!(
                            //     "Generated code for file: {}, form: {}, field: {}, rule: {}",
                            //     file_name,
                            //     form_name,
                            //     field_name,
                            //     self.get_validate_type(&rule)
                            // );
                            // println!("result: {:?}", result);
                        } else {
                            eprintln!(
                                "Template not found for language: {}, validate type: {}",
                                self.get_language(),
                                self.get_validate_type(&rule)
                            );
                        }
                    }

                    let output_path =
                        format!("{}/{}.{}", output_dir, file_name, self.get_extension());

                    std::fs::write(&output_path, final_result)
                        .expect("Failed to write output file");
                }
            }
        }
    }

    fn check_template_exits(&self, language: &String, validate_type: &String) -> bool {
        let template_path = format!(
            "./crates/compiler/src/templates/{}/{}.tera",
            language, validate_type
        );
        println!("Checking if template exists: {}", template_path);
        std::path::Path::new(&template_path).exists()
    }
    // crates\compiler\src\templates\javascript\string.tera
    // ./templates/javascript/string.tera
    fn get_context(
        &self,
        field_name: &String,
        path: &String,
        required: bool,
        default_error: &String,
        rule: &Rule,
        transform: Option<&Transform>,
    ) -> Context {
        let mut context = Context::new();
        context.insert("FieldName", field_name);
        context.insert("FieldNamePath", path);
        context.insert("required", &required);
        context.insert("requiredError", default_error);
        context.insert("defaultError", default_error);

        // Tera templates access rule/transform fields directly (e.g. rules.minLength.value,
        // transform.trim). That doesn't work when passing Rust enums, so pass the inner structs.
        let rules: Option<serde_json::Value> = match rule {
            Rule::String(rules) => Some(serde_json::to_value(rules).unwrap()),
            Rule::Number(rules) => Some(serde_json::to_value(rules).unwrap()),
            Rule::Boolean(rules) => Some(serde_json::to_value(rules).unwrap()),
            Rule::Array(rules) => Some(serde_json::to_value(rules).unwrap()),
            Rule::File(rules) => Some(serde_json::to_value(rules).unwrap()),
            Rule::Enum(rules) => Some(serde_json::to_value(rules).unwrap()),
            _ => None,
        };
        context.insert("rules", &rules);

        let string_transform: Option<serde_json::Value> = match transform {
            Some(Transform::String(t)) => {
                let mut t = t.clone();
                t.cast = match &t.cast {
                    Some(FieldType::Number) => Some(FieldType::Number),
                    Some(FieldType::Boolean) => Some(FieldType::Boolean),
                    _ => None,
                };
                Some(serde_json::to_value(t).unwrap())
            }
            Some(Transform::Number(t)) => {
                let mut t = t.clone();
                t.cast = match &t.cast {
                    Some(FieldType::String) => Some(FieldType::String),
                    Some(FieldType::Boolean) => Some(FieldType::Boolean),
                    _ => None,
                };
                Some(serde_json::to_value(t).unwrap())
            }
            Some(Transform::Boolean(t)) => {
                let mut t = t.clone();
                t.cast = match &t.cast {
                    Some(FieldType::String) => Some(FieldType::String),
                    Some(FieldType::Number) => Some(FieldType::Number),
                    _ => None,
                };
                Some(serde_json::to_value(t).unwrap())
            }
            Some(Transform::Array(t)) => {
                let mut t = t.clone();
                t.cast = match &t.cast {
                    Some(FieldType::String) => Some(FieldType::String),
                    Some(FieldType::Number) => Some(FieldType::Number),
                    Some(FieldType::Boolean) => Some(FieldType::Boolean),
                    _ => None,
                };
                Some(serde_json::to_value(t).unwrap())
            }
            Some(Transform::File(t)) => {
                let mut t = t.clone();
                t.cast = match &t.cast {
                    Some(FieldType::String) => Some(FieldType::String),
                    Some(FieldType::Number) => Some(FieldType::Number),
                    Some(FieldType::Boolean) => Some(FieldType::Boolean),
                    _ => None,
                };
                Some(serde_json::to_value(t).unwrap())
            }
            Some(Transform::Enum(t)) => {
                let mut t = t.clone();
                t.cast = match &t.cast {
                    Some(FieldType::String) => Some(FieldType::String),
                    Some(FieldType::Number) => Some(FieldType::Number),
                    Some(FieldType::Boolean) => Some(FieldType::Boolean),
                    _ => None,
                };
                Some(serde_json::to_value(t).unwrap())
            }
            _ => None,
        };

        context.insert("transform", &string_transform);

        info!("Context for field '{}': {:?}", field_name, context);

        context
    }

    fn get_language(&self) -> String;
    fn get_extension(&self) -> String;

    fn get_validate_type(&self, rule: &Rule) -> String;
}

struct JavascriptTemplater {
    language: String,
    extension: String,
}

impl Templater for JavascriptTemplater {
    fn new() -> Self {
        Self {
            language: "javascript".to_string(),
            extension: "js".to_string(),
        }
    }

    fn get_language(&self) -> String {
        self.language.clone()
    }

    fn get_extension(&self) -> String {
        self.extension.clone()
    }

    fn get_validate_type(&self, rule: &Rule) -> String {
        match rule {
            Rule::String(_) => "string".to_string(),
            Rule::Number(_) => "number".to_string(),
            Rule::Boolean(_) => "boolean".to_string(),
            Rule::Array(_) => "array".to_string(),
            Rule::File(_) => "file".to_string(),
            Rule::Enum(_) => "enum".to_string(),
            //todo[Add]: Type
            _ => panic!("Unsupported rule type for JavascriptTemplater"),
        }
    }
}
