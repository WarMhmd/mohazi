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

        // This is like coordinates
        let line_index = position.line as usize;
        let col_index = position.character as usize;

        // Safely get the line
        let line = match content.lines().nth(line_index) {
            Some(l) => l,
            None => return Ok(None),
        };

        // 3. Analyze context (The "Heuristic")
        // Get text up to the cursor
        let safe_col = col_index.min(line.len());
        let prefix = &line[..safe_col];

        // CASE A: User is typing a value for "type: "
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

        // CASE B: User is starting a new rule (Top level or field level)
        // You might check indentation here to know if you are inside a field
        return Ok(Some(CompletionResponse::Array(completion_items![
            ("rules:", "Define validation rules"),
            ("transform:", "Transform block"),
        ])));
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
        eprintln!("LSP: Validate called for {}", uri);
        eprintln!("LSP: Text length: {}", text.len());

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
}
