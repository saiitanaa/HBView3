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
    top_pixels: Vec<PixelData>,
    bottom_pixels: Vec<PixelData>,
    top_rectangles: Vec<RectData>,
    bottom_rectangles: Vec<RectData>,
    top_background: u32,
    bottom_background: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PixelData {
    x: usize,
    y: usize,
    color: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct RectData {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
}

enum PreviewNotification {}

impl tower_lsp::lsp_types::notification::Notification for PreviewNotification {
    type Params = PreviewParams;

    const METHOD: &'static str = "hbview3/preview";
}

struct Backend {
    client: Client,
}

impl Backend {
    fn build_preview(runtime: &runtime::Runtime) -> PreviewParams {
        let top = runtime.top_screen();
        let bottom = runtime.bottom_screen();

        PreviewParams {
            top_text: top
                .text
                .iter()
                .map(|line| line.text.clone())
                .collect(),

            bottom_text: bottom
                .text
                .iter()
                .map(|line| line.text.clone())
                .collect(),

            top_pixels: top
                .pixels
                .iter()
                .map(|pixel| PixelData {
                    x: pixel.x,
                    y: pixel.y,
                    color: pixel.color,
                })
                .collect(),

            bottom_pixels: bottom
                .pixels
                .iter()
                .map(|pixel| PixelData {
                    x: pixel.x,
                    y: pixel.y,
                    color: pixel.color,
                })
                .collect(),

            top_rectangles: top
                .rectangles
                .iter()
                .map(|rect| RectData {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height,
                    color: rect.color,
                })
                .collect(),

            bottom_rectangles: bottom
                .rectangles
                .iter()
                .map(|rect| RectData {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height,
                    color: rect.color,
                })
                .collect(),

            top_background: top.background_color,
            bottom_background: bottom.background_color,
        }
    }
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
                        format!(
                            "Failed to convert document URI: {error:?}"
                        ),
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
                        format!(
                            "Failed to parse {}: {error}",
                            path.display()
                        ),
                    )
                    .await;

                return;
            }
        };

        let mut runtime = runtime::Runtime::new();
        runtime.execute(&program);

        let preview = Self::build_preview(&runtime);

        self.client
            .send_notification::<PreviewNotification>(preview)
            .await;
    }

    async fn did_change(
        &self,
        params: DidChangeTextDocumentParams,
    ) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "HBView3 did_change: {}",
                    params.text_document.uri
                ),
            )
            .await;

        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Changes received: {}",
                    params.content_changes.len()
                ),
            )
            .await;

        let Some(change) = params.content_changes.first() else {
            return;
        };

        let path = match params.text_document.uri.to_file_path() {
            Ok(path) => path,
            Err(error) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!(
                            "Failed to convert document URI: {error:?}"
                        ),
                    )
                    .await;

                return;
            }
        };

        let program = match parser::parse_source(&path, &change.text) {
            Ok(program) => program,
            Err(error) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!(
                            "Failed to parse {}: {error}",
                            path.display()
                        ),
                    )
                    .await;

                return;
            }
        };

        let mut runtime = runtime::Runtime::new();
        runtime.execute(&program);

        let preview = Self::build_preview(&runtime);

        self.client
            .send_notification::<PreviewNotification>(preview)
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