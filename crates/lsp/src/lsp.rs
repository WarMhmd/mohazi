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

        match context.as_deref() {
            Some("rules") => {
                return Ok(Some(CompletionResponse::Array(completion_items![
                    ("minLength", "Rule: Minimum string length"),
                    ("maxLength", "Rule: Maximum string length"),
                    ("min", "Rule: Minimum value"),
                    ("max", "Rule: Maximum value"),
                    ("regex", "Rule: Match custom regex pattern"),
                    ("pattern", "Rule: Match custom regex pattern"),
                ])));
            }
            Some("transform") => {
                return Ok(Some(CompletionResponse::Array(completion_items![
                    ("cast", "Transform: Cast type"),
                    ("trim", "Transform: Remove whitespace"),
                    ("join", "Transform: Join array elements"),
                    ("split", "Transform: split array elements"),
                    ("lowercase", "Transform: Convert to lowercase"),
                    ("uppercase", "Transform: Convert to uppercase"),
                ])));
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

                return Ok(Some(CompletionResponse::Array(completion_items![
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

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
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
}
