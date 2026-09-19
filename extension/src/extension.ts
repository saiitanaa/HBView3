import * as path from "path";
import * as vscode from "vscode";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let previewPanel: vscode.WebviewPanel | undefined;

let latestPreview: PreviewData | undefined;

interface PreviewData {
    top_text: string[];
    bottom_text: string[];

    top_pixels: PixelData[];
    bottom_pixels: PixelData[];

    top_rectangles: RectData[];
    bottom_rectangles: RectData[];

    top_background: number;
    bottom_background: number;
}

interface PixelData {
    x: number;
    y: number;
    color: number;
}

interface RectData {
    x: number;
    y: number;
    width: number;
    height: number;
    color: number;
}

export function activate(context: vscode.ExtensionContext) {
    const serverPath = path.join(
        context.extensionPath,
        "..",
        "server",
        "target",
        "debug",
        "hbview3-server",
    );

    const serverOptions: ServerOptions = {
        run: {
            command: serverPath,
        },
        debug: {
            command: serverPath,
        },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: "file", language: "c" },
            { scheme: "file", language: "cpp" },
        ],
    };

    const client = new LanguageClient(
        "HBView3",
        "HBView3 Language Server",
        serverOptions,
        clientOptions,
    );

    client.onNotification(
        "hbview3/preview",
        (params: PreviewData) => {
            latestPreview = params;

            previewPanel?.webview.postMessage({
                type: "preview",
                data: params,
            });
        },
    );

    context.subscriptions.push(client);

    console.log("HBView3 server:", serverPath);

    client.start();

    const command = vscode.commands.registerCommand(
        "hbview3.openPreview",
        async () => {
            const files = await vscode.window.showOpenDialog({
                canSelectFiles: true,
                canSelectFolders: false,
                canSelectMany: false,
                filters: {
                    "C/C++": ["c", "cpp", "cc", "cxx"],
                },
            });

            if (!files || files.length === 0) {
                return;
            }

            const file = files[0];

            const document =
                await vscode.workspace.openTextDocument(file);

            previewPanel?.dispose();

            previewPanel = vscode.window.createWebviewPanel(
                "hbview3Preview",
                "HBView3",
                vscode.ViewColumn.Beside,
                {
                    enableScripts: true,
                },
            );

            previewPanel.webview.onDidReceiveMessage(
                (message) => {
                    if (message.type !== "ready") {
                        return;
                    }

                    console.log("HBView3 Webview ready");

                    if (!latestPreview) {
                        return;
                    }

                    previewPanel?.webview.postMessage({
                        type: "preview",
                        data: latestPreview,
                    });
                },
                undefined,
                context.subscriptions,
            );

            previewPanel.webview.html =
                getPreviewHtml(file.fsPath);

            await vscode.window.showTextDocument(document, {
                preview: false,
            });
        },
    );

    context.subscriptions.push(command);
}

function getPreviewHtml(filePath: string): string {
    return `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">

    <style>
        html,
        body {
            width: 100%;
            height: 100%;
            margin: 0;
            padding: 0;
            overflow: hidden;
            background: #111;
        }

        body {
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .console {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
        }

        canvas {
            display: block;
            image-rendering: pixelated;
            background: black;
        }

        #top {
            width: 400px;
            height: 240px;
        }

        #bottom {
            width: 320px;
            height: 240px;
        }

        .file {
            position: fixed;
            top: 8px;
            left: 8px;
            font-family: monospace;
            font-size: 12px;
            color: #888;
        }
    </style>
</head>

<body>
    <div class="file">${escapeHtml(filePath)}</div>

    <div class="console">
        <canvas
            id="top"
            width="400"
            height="240"
        ></canvas>

        <canvas
            id="bottom"
            width="320"
            height="240"
        ></canvas>
    </div>

    <script>
        const vscode = acquireVsCodeApi();

        const topCanvas =
            document.getElementById("top");

        const bottomCanvas =
            document.getElementById("bottom");

        const topContext =
            topCanvas.getContext("2d");

        const bottomContext =
            bottomCanvas.getContext("2d");

        topContext.imageSmoothingEnabled = false;
        bottomContext.imageSmoothingEnabled = false;

        function colorToCss(color) {
            const r = (color >> 16) & 0xff;
            const g = (color >> 8) & 0xff;
            const b = color & 0xff;

            return "rgb(" + r + "," + g + "," + b + ")";
        }

        function renderScreen(
            context,
            width,
            height,
            background,
            pixels,
            rectangles,
            text
        ) {
            context.clearRect(
                0,
                0,
                width,
                height
            );

            context.fillStyle =
                colorToCss(background);

            context.fillRect(
                0,
                0,
                width,
                height
            );

            for (const rect of rectangles) {
                context.fillStyle =
                    colorToCss(rect.color);

                context.fillRect(
                    rect.x,
                    rect.y,
                    rect.width,
                    rect.height
                );
            }

            for (const pixel of pixels) {
                context.fillStyle =
                    colorToCss(pixel.color);

                context.fillRect(
                    pixel.x,
                    pixel.y,
                    1,
                    1
                );
            }

            context.fillStyle = "white";
            context.font = "10px monospace";

            for (
                let index = 0;
                index < text.length;
                index++
            ) {
                context.fillText(
                    text[index],
                    8,
                    16 + index * 16
                );
            }
        }

        window.addEventListener(
            "message",
            (event) => {
                const message = event.data;

                if (message.type !== "preview") {
                    return;
                }

                const data = message.data;

                renderScreen(
                    topContext,
                    400,
                    240,
                    data.top_background,
                    data.top_pixels,
                    data.top_rectangles,
                    data.top_text
                );

                renderScreen(
                    bottomContext,
                    320,
                    240,
                    data.bottom_background,
                    data.bottom_pixels,
                    data.bottom_rectangles,
                    data.bottom_text
                );
            }
        );

        vscode.postMessage({
            type: "ready",
        });
    </script>
</body>
</html>
`;
}

function escapeHtml(value: string): string {
    return value
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;")
        .replaceAll("'", "&#039;");
}

export function deactivate() {}