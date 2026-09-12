import * as path from "path";
import * as vscode from "vscode";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let previewPanel: vscode.WebviewPanel | undefined;

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
        (params: { text: string }) => {
            console.log("HBView3 received source:", params.text);

            previewPanel?.webview.postMessage({
                type: "source",
                text: params.text,
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
            const document = await vscode.workspace.openTextDocument(file);

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

                    previewPanel?.webview.postMessage({
                        type: "source",
                        text: document.getText(),
                    });
                },
                undefined,
                context.subscriptions,
            );

            previewPanel.webview.html = getPreviewHtml(file.fsPath);

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
        <canvas id="top" width="400" height="240"></canvas>
        <canvas id="bottom" width="320" height="240"></canvas>
    </div>

    <script>
        const vscode = acquireVsCodeApi();

        const topCanvas = document.getElementById("top");
        const bottomCanvas = document.getElementById("bottom");

        const topContext = topCanvas.getContext("2d");
        const bottomContext = bottomCanvas.getContext("2d");

        topContext.imageSmoothingEnabled = false;
        bottomContext.imageSmoothingEnabled = false;

        topContext.fillStyle = "red";
        topContext.fillRect(0, 0, 400, 240);

        bottomContext.fillStyle = "blue";
        bottomContext.fillRect(0, 0, 320, 240);

        window.addEventListener("message", (event) => {
            const message = event.data;

            if (message.type !== "source") {
                return;
            }

            topContext.fillStyle = "red";
            topContext.fillRect(0, 0, 400, 240);

            topContext.fillStyle = "white";
            topContext.font = "10px monospace";

            const lines = message.text.split("\\n");

            lines.slice(0, 22).forEach((line, index) => {
                topContext.fillText(
                    line.slice(0, 62),
                    4,
                    12 + index * 10,
                );
            });
        });

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