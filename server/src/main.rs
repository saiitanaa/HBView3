mod ir;
mod parser;
mod runtime;
mod screen;
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
    top_text: Vec<String>,
    bottom_text: Vec<String>,
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
                format!(
                    "HBView3 opened: {}",
                    params.text_document.uri
                ),
            )
            .await;

        let path = match params.text_document.uri.to_file_path() {
            Ok(path) => path,
            Err(error) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to convert document URI: {error:?}"),
                    )
                    .await;

                return;
            }
        };

        let program = match parser::parse_source(&path, &text) {
            Ok(program) => program,
            Err(error) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to parse {}: {error}", path.display()),
                    )
                    .await;

                return;
            }
        };

        let mut runtime = runtime::Runtime::new();
        runtime.execute(&program);

        let top_text = runtime
            .top_screen()
            .text
            .iter()
            .map(|line| line.text.clone())
            .collect();

        let bottom_text = runtime
            .bottom_screen()
            .text
            .iter()
            .map(|line| line.text.clone())
            .collect();

        self.client
            .send_notification::<PreviewNotification>(
                PreviewParams {
                    top_text,
                    bottom_text,
                },
            )
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::{parser, runtime};
    use std::path::Path;

    #[test]
    fn parses_main() {
        let source = r#"
#include <3ds.h>
#include <stdio.h>

int main() {
    gfxInitDefault();
    consoleInit(GFX_TOP, nullptr);
    printf("Hello 3DS!");
    return 0;
}
"#;

        let program =
            parser::parse_source(Path::new("main.cpp"), source)
                .expect("Failed to parse");

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "main");
    }

    #[test]
    fn executes_main() {
        let source = r#"
#include <3ds.h>
#include <stdio.h>

int main() {
    gfxInitDefault();
    consoleInit(GFX_TOP, nullptr);
    printf("Hello 3DS!");
    return 0;
}
"#;

        let program =
            parser::parse_source(Path::new("main.cpp"), source)
                .expect("Failed to parse");

        let mut runtime = runtime::Runtime::new();

        runtime.execute(&program);

        assert_eq!(
            runtime.top_screen().text[0].text,
            "Hello 3DS!"
        );
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