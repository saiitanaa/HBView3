<img width="28346" height="3402" alt="banner" src="https://github.com/user-attachments/assets/e81fc379-b2f2-4357-a252-9086dc9946ae" />

## Previewer for Nintendo 3DS homebrew development.

HBView3 lets you preview real 3DS homebrew projects on your desktop without rebuilding and launching the project on a 3DS for every UI change.

HBView3 reads real C/C++ source code directly and provides a virtual 3DS environment for previewing supported functionality.

# Status : v0.0.12-dev 🪛

## Example

https://github.com/user-attachments/assets/94784341-2a60-4457-8258-7dbefc3a19e5

## Usage

Download the latest release for your platform from the GitHub Releases page.

### macOS / Linux

Extract the archive, then make the binary executable:

```bash
chmod +x HBView3
```

Run it with the path to your 3DS homebrew project:

```bash
./HBView3 /path/to/project
```

For example:

```bash
./HBView3 ~/Projects/MyHomebrew
```

### Windows

Extract the ZIP archive and open PowerShell in the extracted directory:

```powershell
.\HBView3.exe C:\path\to\project
```

For example:

```powershell
.\HBView3.exe C:\Projects\MyHomebrew
```

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
      egui/eframe
        ↓
 Desktop preview
```

The previewer does not compile the homebrew for every change. It parses the source and executes the supported functionality inside the virtual runtime.

## Status

HBView3 is currently under development.

Currently supported functionality includes:

* C/C++ source parsing
* 3DS project discovery
* Virtual top and bottom screens
* Native 3DS screen dimensions
* Console text rendering
* `printf`
* Hot reload
* egui/eframe rendering

More 3DS functionality will be added over time.
