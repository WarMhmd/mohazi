use std::collections::HashMap;
use std::sync::RwLock;

use core_lib::vis_parser;
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

        let current_field_type = self.get_current_field_type(&lines, line_index).unwrap_or_else(|| { return String::from("")});

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
                            ("startsWith", "Rule: Check if the string starts with a prefix"),
                            ("endsWith", "Rule: Check if the string ends with a suffix"),
                            ("uppercase", "Rule: Check if the string is written in uppercase"),
                            ("lowercase", "Rule: Check if the string is written in lowercase"),
                        ])));
                    }
                    "number" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("min", "Rule: Minimum value"),
                            ("max", "Rule: Maximum value"),
                            ("gt", "Rule:  Greater than"),
                            ("gte", "Rule:  Greater than or equal"),
                            ("lt", "Rule:  Less than"),
                            ("lte", "Rule:  Less than or equal"),
                            ("equal", "Rule: Check if the number is equal to some value"),
                            ("positive", "Rule: Check if the number is positive"),
                            ("negative", "Rule: Check if the number is negative"),
                            ("nonpositive", "Rule: Check if the number is not positive"),
                            ("nonnegative", "Rule: Check if the number is not negative"),
                            ("multipleOf", "Rule: Check if the number is a multiple of some value"),
                        ])));
                    }
                    "file" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("extension", "Rule: Check if the file has a specific extension"),
                            ("maxSize", "Rule: Check if the file size is less than or equal to some value"),
                            ("minSize", "Rule: Check if the file size is greater than or equal to some value"),
                        ])));
                    }
                    "boolean" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("state", "Rule: Check if the boolean is true or false"),
                        ])));
                    }
                    "enum" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("values", "Rule: Check if the enum value is one of the allowed values"),
                        ])));
                    }
                    "array" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("type", "Rule: Check if the array elements are of a specific type"),
                            ("minLength", "Rule: Check if the array length is greater than or equal to some value"),
                            ("maxLength", "Rule: Check if the array length is less than or equal to some value"),
                            ("length", "Rule: Check if the array length is equal to some value"),
                        ])));
                    }
                    "" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("random", "Random"),
                        ])));
                    }
                    _ => unreachable!()
                }
            }
            Some("transform") => {
                match current_field_type.as_str() {
                    "string" => {
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
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                        ])));
                    }
                    "file" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                        ])));
                    }
                    "boolean" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                        ])));
                    }
                    "array" => {
                        return Ok(Some(CompletionResponse::Array(completion_items![
                            ("cast", "Transform: Cast type"),
                            ("join", "Transform: Join array elements"),
                            ("sum", "Transform: Sum array elements"),
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
                    _ => unreachable!()
                }
            }
            _ => {
                // Top level context
                if prefix.trim().ends_with("type:") || prefix.trim().ends_with("field_type:") {
                    // check the macro implementation to understand
                    // todo[Add]: type
                    return Ok(Some(CompletionResponse::Array(completion_items![
                        ("string", "String type"),
                        ("number", "Number type"),
                        ("boolean", "Boolean type"),
                        ("array", "Array type"),
                        ("enum", "Enum type"),
                        ("file", "File type"),
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
            "type" => "## Keyword: type\nDefines the data type of the field. \n\n**Usage:** `type: string`",
            "required" => "## Keyword: Required**\n`required: true`\n\nMarks the field as mandatory.",
            "defaultError" => "## Keyword: Default error message**\n`defaultError: 'Error message'`\n\nSets a default error message to be used when the field is missing.",
            "rules" => "## Keyword: rules\nDefines validation rules for a specific field.",
            "transform" => "## Keyword: rules\nDefines available transforms a field.",
            
            // Types
            // todo[Add]: type
            "string" => "**Type: String**\nRepresents text data.",
            "number" => "**Type: Number**\nRepresents a number.",
            "file" => "**Type: File**\nA file field.",
            "array" => "**Type: Array**\nA list of elements field.",
            "boolean" => "**Type: Boolean**\nA field of either true or false.",
            "enum" => "**Type: Enum**\nA field with a limited set of possible values.",
            
            // Rules 
            "min" => "**Rule: Min**\n`min: <number>`\n\nCorrect types: number.\n\nEnforces a minimum numeric value or string length.",
            "min" => "**Rule: Max**\n`max: <number>`\n\nCorrect types: number.\n\nEnforces a maximum numeric value or string length.",
            "maxLength" => "**Rule: Max length**\n`maxLength: <number>`\n\nCorrect types: string.\n\nEnforces a maximum string length.",
            "minLength" => "**Rule: Min length**\n`minLength: <number>`\n\nCorrect types: string.\n\nEnforces a minimum string length.",

            // Transforms
            "cast" => "**Transform: Cast**\n`cast: <type>`\n\nConverts the field to the specified type.",
            "trim" => "**Transform: Trim**\n`trim: true`\n\nRemoves leading and trailing whitespace from the value.",
            "lowercase" => "**Transform: Lowercase**\n`lowercase: true`\n\nConverts the value to lowercase.",
            "uppercase" => "**Transform: Uppercase**\n`uppercase: true`\n\nConverts the value to uppercase.",
            
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
                    target_indent = indent;
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

            if trimmed_line.starts_with("type:") || trimmed_line.starts_with("cast:") {
                parsing_type = line.splitn(2, ":").last().unwrap().trim().to_string();
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
