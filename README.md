<img width="28346" height="3402" alt="banner" src="https://github.com/user-attachments/assets/e81fc379-b2f2-4357-a252-9086dc9946ae" />

## VS Code / VSCodium extension for previewing Nintendo 3DS homebrew projects ( CURRENTLY IN DEVELOPMENT ) 

HBView3 lets you preview real 3DS homebrew projects directly inside your editor, without rebuilding and launching the project on a 3DS (or in an emulator) for every UI change.

HBView3 reads real C/C++ source code directly and provides a virtual 3DS environment for previewing supported functionality, right next to your code.

## Installation

HBView3 is distributed as an extension for both **VS Code** and **VSCodium**.

### VS Code

Install it from the [Visual Studio Code Marketplace](#), or from the Extensions panel by searching for `HBView3`.

### VSCodium

VSCodium uses [Open VSX](https://open-vsx.org/) instead of the Microsoft Marketplace. Install it from Open VSX, or from the Extensions panel by searching for `HBView3`.

### Manual install (.vsix)

You can also grab the latest `.vsix` from the [GitHub Releases](../../releases) page and install it manually:

```bash
code --install-extension hbview3-x.y.z.vsix
```

```bash
codium --install-extension hbview3-x.y.z.vsix
```

## Usage

1. Open your 3DS homebrew project folder in VS Code / VSCodium.
2. Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and run **HBView3: Preview Project**.
3. The virtual 3DS preview opens in a side panel and stays in sync with your source files.

For example, opening a project at `~/Projects/MyHomebrew` and running the command will detect `main.c`, `source/`, `include/`, and the `Makefile`, then start the preview.

## Project Requirements

HBView3 works with real C/C++ Nintendo 3DS homebrew projects.

A typical project can look like:

```text
MyHomebrew/
├── main.c
├── source/
├── include/
├── assets/
└── Makefile
```

HBView3 parses the C/C++ source directly and previews supported 3DS functionality without compiling the homebrew.

The original project can still be built normally with devkitARM and the Nintendo 3DS development tools.

## Hot Reload

HBView3 automatically detects changes to C/C++ source files.

Edit your source:

```c
printf("Hello HBView3!");
```

Save the file, and the preview is automatically reloaded.

No rebuild is required.

## Virtual 3DS Screens

HBView3 provides two virtual Nintendo 3DS screens using their native dimensions:

```text
Top screen:    400 × 240
Bottom screen: 320 × 240
```

The screens are displayed together and scaled while preserving their original proportions.

## Architecture

```text
Real C/C++ source
        ↓
    Tree-sitter
        ↓
     HBView3 IR
        ↓
  Virtual 3DS runtime
        ↓
   Extension webview
        ↓
   Editor preview
```

The previewer does not compile the homebrew for every change. It parses the source and executes the supported functionality inside the virtual runtime, rendered in the editor's webview.
* Webview-based rendering

More 3DS functionality will be added over time.
