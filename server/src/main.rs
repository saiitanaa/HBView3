mod ir;
mod parser;
use tower_lsp::{
    lsp_types::*,
    Client,
    LanguageServer,
    LspService,
    Server,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PreviewParams {
    text: String,
}

enum PreviewNotification {}

impl tower_lsp::lsp_types::notification::Notification for PreviewNotification {
    type Params = PreviewParams;

    const METHOD: &'static str = "hbview3/preview";
}

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(
                    TextDocumentSyncCapability::Kind(
                        TextDocumentSyncKind::FULL,
                    ),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(
        &self,
    ) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(
        &self,
        params: DidOpenTextDocumentParams,
    ) {
        let text = params.text_document.text;

        self.client
            .log_message(
                MessageType::INFO,
                format!("HBView3 opened: {}", params.text_document.uri),
            )
            .await;

        self.client
            .log_message(
                MessageType::INFO,
                format!("HBView3 received {} bytes", text.len()),
            )
            .await;

        self.client
            .send_notification::<PreviewNotification>(
                PreviewParams {
                    text,
                },
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
    });

    Server::new(stdin, stdout, socket)
        .serve(service)
        .await;
}