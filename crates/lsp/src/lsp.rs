use std::collections::HashMap;
use std::sync::RwLock;

use core_lib::ast::Transform;
use core_lib::vis_parser;
use include_dir::{include_dir, Dir};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

// This macro is used to remove some boilerplate
macro_rules! completion_items {
    ( $( ($label:expr, $detail:expr) ),* $(,)? ) => {
        vec![
            $(
                CompletionItem::new_simple($label.to_string(), $detail.to_string()),
            )*
        ]
    };
}

/// A helper function that returns how many whitespaces are at the beginning of a line
fn get_indent_level(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

static DOCS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/docs");

fn get_docs(dir: &str, name: &str) -> Option<&'static str> {
    let path = format!("{}/{}.md", dir, name);
    DOCS_DIR.get_file(path)?.contents_utf8()
}

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub documents: RwLock<HashMap<Url, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![":".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        {
            // This should be in a block to make sure the lock is destroyed before going on
            let mut docs = self.documents.write().unwrap(); // writes the documents to the backend
            docs.insert(
                params.text_document.uri.clone(),
                params.text_document.text.clone(),
            );
        }

        let text = params.text_document.text;
        self.validate_file(params.text_document.uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // Since we requested FULL sync, the first change event has the complete text
        if let Some(change) = params.content_changes.into_iter().next() {
            {
                // This should be in a block to make sure the lock is destroyed before going on
                let mut docs = self.documents.write().unwrap();
                docs.insert(params.text_document.uri.clone(), change.text.clone());
            }

            self.validate_file(params.text_document.uri, change.text)
                .await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let docs = self.documents.read().unwrap(); // Reading the document from the backend
        let content = match docs.get(&uri) {
            // Takes the current buffer doc
            Some(text) => text,
            None => return Ok(None),
        };

        let lines: Vec<&str> = content.lines().collect();

        // This is like coordinates
        let line_index = position.line as usize;
        let col_index = position.character as usize;

        // for safety
        if line_index >= lines.len() {
            return Ok(None);
        }

        // we find what context are we in to show correct completion
        let context = self.find_parent_context(&lines, line_index);

        // Safely get the line
        let line = match content.lines().nth(line_index) {
            Some(l) => l,
            None => return Ok(None),
        };

        // 3. Analyze context (The "Heuristic")
        // Get text up to the cursor
        let safe_col = col_index.min(line.len());
        let prefix = &line[..safe_col];

        let current_field_type = self
            .get_current_field_type(&lines, line_index)
            .unwrap_or_else(|| return String::from(""));

        match context.as_deref() {
            Some("rules") => {
                match current_field_type.as_str() {
                    "string" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("minLength", "Rule: Minimum string length"),
                            ("maxLength", "Rule: Maximum string length"),
                            ("length", "Rule: Set the string length"),
                            ("regex", "Rule: Match custom regex pattern"),
                            ("pattern", "Rule: Match custom regex pattern"),
                            (
                                "startsWith",
                                "Rule: Check if the string starts with a prefix"
                            ),
                            ("endsWith", "Rule: Check if the string ends with a suffix"),
                            (
                                "uppercase",
                                "Rule: Check if the string is written in uppercase"
                            ),
                            (
                                "lowercase",
                                "Rule: Check if the string is written in lowercase"
                            ),
                            (
                                "error",
                                "Rule Error: set custom error message for the previous rule"
                            ),
                        ])));
                    }
                    "number" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("gt", "Rule:  Greater than"),
                            ("gte", "Rule:  Greater than or equal"),
                            ("lt", "Rule:  Less than"),
                            ("lte", "Rule:  Less than or equal"),
                            ("equal", "Rule: Check if the number is equal to some value"),
                            ("positive", "Rule: Check if the number is positive"),
                            ("negative", "Rule: Check if the number is negative"),
                            ("nonpositive", "Rule: Check if the number is not positive"),
                            ("nonnegative", "Rule: Check if the number is not negative"),
                            (
                                "multipleOf",
                                "Rule: Check if the number is a multiple of some value"
                            ),
                            (
                                "error",
                                "Rule Error: set custom error message for the previous rule"
                            ),
                        ])));
                    }
                    "file" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("extension", "Rule: Check if the file has a specific extension"),
                            ("maxSize", "Rule: Check if the file size is less than or equal to some value"),
                            ("minSize", "Rule: Check if the file size is greater than or equal to some value"),
                            ("error", "Rule Error: set custom error message for the previous rule"),
                        ])));
                    }
                    "boolean" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("state", "Rule: Check if the boolean is true or false"),
                            (
                                "error",
                                "Rule Error: set custom error message for the previous rule"
                            ),
                        ])));
                    }
                    "enum" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            (
                                "values",
                                "Rule: Check if the enum value is one of the allowed values"
                            ),
                            (
                                "error",
                                "Rule Error: set custom error message for the previous rule"
                            ),
                        ])));
                    }
                    "image" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("width", "Rule: Check if the image width is equal to some value"),
                            ("height", "Rule: Check if the image height is equal to some value"),
                            ("minWidth", "Rule: Check if the image width is greater than or equal to some value"),
                            ("maxWidth", "Rule: Check if the image width is less than or equal to some value"),
                            ("minHeight", "Rule: Check if the image height is greater than or equal to some value"),
                            ("maxHeight", "Rule: Check if the image height is less than or equal to some value"),
                            ("ratio", "Rule: Check if the image aspect ratio matches some value (e.g. 1:1, 16:9)"),
                            ("extension", "Rule: Check if the file has a specific extension"),
                            ("mime", "Rule: Check if the file has a specific mime type"),
                            ("maxSize", "Rule: Check if the file size is less than or equal to some value"),
                            ("minSize", "Rule: Check if the file size is greater than or equal to some value"),
                            ("error", "Rule Error: set custom error message for the previous rule"),
                        ])));
                    }
                    "array" => {
                        if prefix.ends_with("type: ") || prefix.ends_with("field_type: ") {
                            // check the macro implementation to understand
                            // todo[Add]: type
                            return Ok(Some(CompletionResponse::Array(completion_items![
                                ("string", "String type"),
                                ("number", "Number type"),
                                ("boolean", "Boolean type"),
                            ])));
                        }
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("type", "Rule: Check if the array elements are of a specific type"),
                            ("minLength", "Rule: Check if the array length is greater than or equal to some value"),
                            ("maxLength", "Rule: Check if the array length is less than or equal to some value"),
                            ("length", "Rule: Check if the array length is equal to some value"),
                            ("error", "Rule Error: set custom error message for the previous rule"),
                        ])));
                    }
                    "" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![(
                            "unknow type",
                            "You have not defined a type"
                        ),])));
                    }
                    _ => unreachable!(),
                }
            }
            Some("transform") => {
                match current_field_type.as_str() {
                    "string" => {
                        // setup the cast completion list
                        if prefix.ends_with("cast: ") {
                            return Ok(Some(CompletionResponse::Array(completion_items![
                                ("number", "Cast: string to number"),
                                ("boolean", "Cast: string to boolean"),
                            ])));
                        }

                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                            ("trim", "Transform: Remove whitespace"),
                            ("toLowerCase", "Transform: Convert to lowercase"),
                            ("toUpperCase", "Transform: Convert to uppercase"),
                            ("normalize", "Transform: Normalize Unicode characters into a specific Unicode Normalization Form"),
                            ("split", "Transform: split array elements"),
                        ])));
                    }
                    "number" => {
                        // setup the cast completion list
                        if prefix.ends_with("cast: ") {
                            return Ok(Some(CompletionResponse::Array(completion_items![
                                ("number", "Cast: number to number"),
                                ("string", "Cast: number to string"),
                                ("boolean", "Cast: number to boolean"),
                                ("hex", "Cast: number to hex"),
                            ])));
                        }

                        return Ok(Some(CompletionResponse::Array(completion_items![(
                            "cast",
                            "Transform: Cast type"
                        ),])));
                    }
                    "file" => {
                        // setup the cast completion list
                        if prefix.ends_with("cast: ") {
                            return Ok(Some(CompletionResponse::Array(completion_items![
                                ("image", "Cast: file to image"),
                                ("base64", "Cast: file to base64"),
                            ])));
                        }

                        return Ok(Some(CompletionResponse::Array(completion_items![(
                            "cast",
                            "Transform: Cast type"
                        ),])));
                    }
                    "boolean" => {
                        // setup the cast completion list
                        if prefix.ends_with("cast: ") {
                            return Ok(Some(CompletionResponse::Array(completion_items![
                                ("number", "Cast: boolean to number"),
                                ("string", "Cast: boolean to string"),
                            ])));
                        }

                        return Ok(Some(CompletionResponse::Array(completion_items![(
                            "cast",
                            "Transform: Cast type"
                        ),])));
                    }
                    "array" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                            ("join", "Transform: Join array elements"),
                            ("sum", "Transform: Sum array elements"),
                        ])));
                    }
                    "image" => {
                        // setup the cast completion list
                        if prefix.ends_with("cast: ") {
                            return Ok(Some(CompletionResponse::Array(completion_items![
                                ("file", "Cast: image to file"),
                                ("base64", "Cast: image to base64"),
                            ])));
                        }

                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                            ("rename", "Transform: Rename image file"),
                        ])));
                    }
                    "" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                            ("trim", "Transform: Remove whitespace"),
                            ("join", "Transform: Join array elements"),
                            ("split", "Transform: split array elements"),
                            ("lowercase", "Transform: Convert to lowercase"),
                            ("uppercase", "Transform: Convert to uppercase"),
                        ])));
                    }
                    _ => unreachable!(),
                }
            }
            _ | None => {
                // Top level context
                if prefix.ends_with("type: ") || prefix.ends_with("field_type: ") {
                    // check the macro implementation to understand
                    // todo[Add]: type
                    return Ok(Some(CompletionResponse::Array(completion_items![
                        ("string", "String type"),
                        ("number", "Number type"),
                        ("boolean", "Boolean type"),
                        ("array", "Array type"),
                        ("enum", "Enum type"),
                        ("file", "File type"),
                        ("image", "Image type"),
                    ])));
                }

                // general keywords for fields
                return Ok(Some(CompletionResponse::Array(completion_items![
                    ("type:", "Set field type"),
                    ("rules:", "Define validation rules"),
                    ("transform:", "Transform block"),
                    ("required:", "Required checking"),
                    ("defaultError:", "Default error message"),
                ])));
            }
        }
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let position = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;

        // get the document
        let doc = self.documents.read().unwrap();
        let content = match doc.get(&uri) {
            Some(text) => text,
            None => return Ok(None),
        };
        let lines = content.lines().collect::<Vec<&str>>();

        let line_index = position.line as usize;
        let line = match content.lines().nth(line_index) {
            Some(l) => l,
            None => return Ok(None),
        };

        // get the current word
        let word = match self.get_word_at_position(line, position.character as usize) {
            Some(w) => w,
            None => return Ok(None), // Cursor is on whitespace
        };

        // documentation table
        let markdown_text = match word.as_str() {
            // Keywords
            "type" => {
                let context = self.find_parent_context(&lines, line_index);

                if context.is_none() {
                    get_docs("keywords", "type").unwrap()
                } else {
                    get_docs("rules", "array-type").unwrap()
                }
            }
            "required" => get_docs("keywords", "required").unwrap(),
            "defaultError" => get_docs("keywords", "defaultError").unwrap(),
            "rules" => get_docs("keywords", "rules").unwrap(),
            "transform" => get_docs("keywords", "transform").unwrap(),

            // Types
            // todo[Add]: type
            "string" => get_docs("types", "string").unwrap(),
            "number" => get_docs("types", "number").unwrap(),
            "file" => get_docs("types", "file").unwrap(),
            "array" => get_docs("types", "array").unwrap(),
            "boolean" => get_docs("types", "boolean").unwrap(),
            "enum" => get_docs("types", "enum").unwrap(),
            "image" => get_docs("types", "image").unwrap(),

            // Rules
            "min" => get_docs("rules", "min").unwrap(),
            "max" => get_docs("rules", "max").unwrap(),
            "maxLength" => get_docs("rules", "maxLength").unwrap(),
            "minLength" => get_docs("rules", "minLength").unwrap(),
            "length" => get_docs("rules", "length").unwrap(),
            "uppercase" => get_docs("rules", "uppercase").unwrap(),
            "lowercase" => get_docs("rules", "lowercase").unwrap(),
            "width" => get_docs("rules", "width").unwrap(),
            "height" => get_docs("rules", "height").unwrap(),
            "minWidth" => get_docs("rules", "minWidth").unwrap(),
            "maxWidth" => get_docs("rules", "maxWidth").unwrap(),
            "minHeight" => get_docs("rules", "minHeight").unwrap(),
            "maxHeight" => get_docs("rules", "maxHeight").unwrap(),
            "ratio" => get_docs("rules", "ratio").unwrap(),
            "mime" => get_docs("rules", "mime").unwrap(),
            // "type" => get_docs("rules", "array-type").unwrap(), // special case, handled in the
            // first type
            "endsWith" => get_docs("rules", "endsWith").unwrap(),
            "startsWith" => get_docs("rules", "startsWith").unwrap(),
            "values" => get_docs("rules", "enum-values").unwrap(),
            "equal" => get_docs("rules", "equal").unwrap(),
            "gt" => get_docs("rules", "gt").unwrap(),
            "lt" => get_docs("rules", "lt").unwrap(),
            "includes" => get_docs("rules", "includes").unwrap(),
            "multipleOf" => get_docs("rules", "multipleOf").unwrap(),
            "negative" => get_docs("rules", "negative").unwrap(),
            "nonnegative" => get_docs("rules", "nonnegative").unwrap(),
            "positive" => get_docs("rules", "positive").unwrap(),
            "nonpositive" => get_docs("rules", "nonpositive").unwrap(),
            "regex" => get_docs("rules", "regex").unwrap(),
            "state" => get_docs("rules", "state").unwrap(),

            // Transforms
            "cast" => get_docs("transforms", "cast").unwrap(),
            "trim" => get_docs("transforms", "trim").unwrap(),
            "join" => get_docs("transforms", "join").unwrap(),
            "split" => get_docs("transforms", "split").unwrap(),
            "sum" => get_docs("transforms", "sum").unwrap(),
            "normalize" => get_docs("transforms", "normalize").unwrap(),
            "toLowerCase" => get_docs("transforms", "toLowerCase").unwrap(),
            "toUpperCase" => get_docs("transforms", "toUpperCase").unwrap(),

            // Fallback (User variables or unknown words)
            _ => return Ok(None),
        };

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(markdown_text.to_string())),
            range: None,
        }))
    }
}

impl Backend {
    async fn validate_file(&self, uri: Url, text: String) {
        let result = vis_parser::parse_vis(&text);

        let errors = match result {
            Ok(_) => vec![],
            Err(e) => e,
        };

        let diagnostics: Vec<Diagnostic> = errors
            .into_iter()
            .map(|err| Diagnostic {
                range: Range {
                    start: Position {
                        line: err.line,
                        character: err.start_col,
                    },
                    end: Position {
                        line: err.line,
                        character: err.end_col,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.message,
                source: Some("verify-lsp".to_string()),
                ..Default::default()
            })
            .collect();

        // publish
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    /// This function is used to now which context the user is writing in (i.e. rules: or transforms:)
    fn find_parent_context(&self, lines: &[&str], current_line_idx: usize) -> Option<String> {
        if current_line_idx == 0 {
            return None;
        }

        // 1. Determine our current indentation target.
        // If we are on a blank line, we assume the user intends to be at the same
        // indentation as the line above, OR we use the cursor column (if available).
        // For simplicity, let's look at the previous non-empty line's indentation.
        let mut target_indent = usize::MAX;

        // 2. Walk backwards
        for i in (0..current_line_idx).rev() {
            let line = lines[i];
            let trim_line = line.trim();

            // Skip empty lines and comments
            if trim_line.is_empty() || trim_line.starts_with('#') {
                continue;
            }

            let indent = get_indent_level(line);

            // Initialize target_indent from the immediate previous line if not set
            if target_indent == usize::MAX {
                // If the current line is indented MORE than the previous line,
                // the previous line IS the parent.
                if indent < get_indent_level(lines[current_line_idx]) {
                    target_indent = indent + 1; // Force a check
                } else {
                    continue;
                }
            }

            // 3. The Logic: The first line we find with strictly LESS indentation
            // than our current block is our Parent.
            if indent < target_indent {
                // Check if this parent line defines a block we care about
                if trim_line.ends_with("rules:") {
                    return Some("rules".to_string());
                } else if trim_line.ends_with("transform:") {
                    return Some("transform".to_string());
                }

                // If we hit a parent that isn't a known block key (e.g. "username:"),
                // we update our target indent and keep searching upwards
                target_indent = indent;
            } else if indent == target_indent {
                // If we hit a parent that isn't a known block key (e.g. "username:"),
                // we set the context to the global context (None)
                return None;
            }
        }

        None
    }

    fn get_word_at_position(&self, line: &str, col: usize) -> Option<String> {
        // If the line is empty or col is out of bounds, return None
        if col >= line.len() {
            return None;
        }

        // 1. Find the start of the word (search backwards)
        // We look for the first character that IS NOT alphanumeric (or underscore)
        let start = line[..col]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1) // Start is the char AFTER the separator
            .unwrap_or(0); // Or 0 if we hit the start of the line

        // 2. Find the end of the word (search forwards)
        let end = line[col..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| col + i) // Offset by current col
            .unwrap_or(line.len()); // Or end of line

        // 3. splice the word
        if start < end {
            Some(line[start..end].to_string())
        } else {
            None
        }
    }

    /// This function gets the current field's type either from the type: or the cast: fields
    fn get_current_field_type(&self, lines: &[&str], current_line_idx: usize) -> Option<String> {
        if current_line_idx == 0 {
            return None;
        }

        let mut parsing_type = String::new();

        // go through previous lines
        for i in (0..current_line_idx).rev() {
            let line = lines[i];
            let trimmed_line = line.trim();

            // two cases for choosing types: 1) direct cast or type 2) indirect transforms (join,
            // split)
            if trimmed_line.starts_with("type:") || trimmed_line.starts_with("cast:") {
                // case 1
                parsing_type = line.splitn(2, ":").last().unwrap().trim().to_string();
                break;
            }

            if trimmed_line.starts_with("join:") {
                // case 2
                parsing_type = "string".to_string();
                break;
            }

            if trimmed_line.starts_with("split:") {
                // case 2
                parsing_type = "array".to_string();
                break;
            }
        }

        if parsing_type.is_empty() {
            return None;
        } else {
            return Some(parsing_type);
        }
    }
}
