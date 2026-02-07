use core::panic;

use core_lib::ast::{Field, Form, Rule, StringRules, StringTransform, Transform};
use indexmap::{map::Entry, IndexMap};
use tera::{Context, Tera};

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
                            println!("transform: {:?}", transform);
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
                            println!(
                                "Generated code for file: {}, form: {}, field: {}, rule: {}",
                                file_name,
                                form_name,
                                field_name,
                                self.get_validate_type(&rule)
                            );
                            println!("result: {:?}", result);
                            // write result to output_dir/filename.extension
                            // let output_path =
                            //     format!("{}/{}.{}", output_dir, file_name, self.get_extension());

                            // std::fs::write(&output_path, result)
                            //     .expect("Failed to write output file");
                        } else {
                            eprintln!(
                                "Template not found for language: {}, validate type: {}",
                                self.get_language(),
                                self.get_validate_type(&rule)
                            );
                        }
                    }
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
        let string_rules: Option<&StringRules> = match rule {
            Rule::String(rules) => Some(rules),
            _ => None,
        };
        context.insert("rules", &string_rules);

        let string_transform: Option<&StringTransform> = match transform {
            Some(Transform::String(t)) => Some(t),
            _ => None,
        };
        context.insert("transform", &string_transform);

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
            _ => panic!("Unsupported rule type for StringTemplater"),
        }
    }
}
