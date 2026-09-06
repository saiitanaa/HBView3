# HBView3

Previewer for Nintendo 3DS homebrew development.

HBView3 lets you preview real 3DS homebrew projects on your desktop without rebuilding and launching the project on a 3DS for every UI change.

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

## Project requirements

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

HBView3 reads the C/C++ source directly and previews the supported 3DS functionality without compiling the homebrew.

## Hot Reload

HBView3 automatically detects changes to C/C++ source files.

Edit your source:

```c
printf("Hello HBView3!");
```

Save the file, and the preview is automatically reloaded.

No rebuild is required.

## Example

[Demo video](https://github.com/user-attachments/assets/b1bacd20-38b5-41bd-b2ed-64d8a84a81dd)

## Status

HBView3 is currently under development.

Supported functionality includes:

* C/C++ source parsing
* 3DS project discovery
* Virtual top and bottom screens
* Console text rendering
* `printf`
* Hot reload
* SDL3 rendering

More 3DS functionality will be added over time.


