# highlight-lib

Cross-platform highlighted text utilities for Rust. This crate lets you read the currently highlighted text in GUI applications and replace it with new text, with platform-specific backends for Windows and Linux.

## Features

- **Read the current selection**: Get the text that is currently selected/highlighted in the focused window.
- **Replace the current selection**: Type replacement text into the focused window, simulating user input.
- **Cross-platform**:
  - **Windows**: Uses UI Automation APIs via the `windows` crate.
  - **Linux**: Tries multiple clipboard/selection tools and typing backends, with special handling for WSL.
  - **Other OSes**: Explicitly panics with clear error messages so unsupported platforms fail loudly.

## Supported platforms

- **Windows**:
  - Uses UI Automation to find the focused element or search the UI tree.
  - Reads selected text via `IUIAutomationTextPattern` or falls back to a control's value via `IUIAutomationValuePattern`.
  - Replaces text using `SendInput` to type Unicode characters directly.
- **Linux**:
  - Tries to read highlighted text from several backends:
    - On WSL, first tries `powershell.exe -NoProfile -Command Get-Clipboard` (normalizing `\r\n` to `\n`).
    - Then `wl-paste -p` (Wayland primary selection).
    - Then `wl-paste` (Wayland clipboard).
    - Then `xclip -o -selection primary` (X11 primary selection).
    - Then `xclip -o` (X11 clipboard).
    - Then `xsel -o` (X11 primary selection).
    - Then `xsel -o -b` (X11 clipboard).
  - For replacement, it:
    - Prefers `wtype -- <text>` when available.
    - Falls back to `xdotool type --clearmodifiers -- <text>` when available.
    - On WSL without a GUI typing tool, returns an error indicating that typing is not supported.
    - Otherwise returns an error indicating that no typing backend (`wtype`/`xdotool`) is available.
- **Other OSes**:
  - `get_highlighted_text` and `replace_highlighted_text` both panic with messages of the form:
    - `"get_highlighted: get_highlighted_text is not implemented for this OS"`
    - `"get_highlighted: replace_highlighted_text is not implemented for this OS"`

## Installation

Add `highlight-lib` to your `Cargo.toml`:

```toml
[dependencies]
highlight-lib = "0.1"
```

The crate targets Rust **edition 2024**.

In your Rust code, the crate name is available as `highlight_lib` (hyphens become underscores):

```rust
use highlight_lib::{get_highlighted_text, replace_highlighted_text};
```

## Quickstart

### Print the current highlighted text

```rust
use highlight_lib::get_highlighted_text;

fn main() {
    match get_highlighted_text() {
        Some(text) => println!("Highlighted text: {text}"),
        None => eprintln!("No highlighted text found or no supported backend."),
    }
}
```

### Replace the current highlighted text

```rust
use highlight_lib::{get_highlighted_text, replace_highlighted_text};

fn main() {
    let Some(original) = get_highlighted_text() else {
        eprintln!("No highlighted text to replace.");
        return;
    };

    let replacement = original.to_uppercase();

    match replace_highlighted_text(&replacement) {
        Ok(()) => println!("Replaced highlighted text with: {replacement}"),
        Err(err) => eprintln!("Failed to replace highlighted text: {err}"),
    }
}
```

## API overview

The public API exposed from the crate root is:

```rust
pub fn get_highlighted_text() -> Option<String>;
pub fn replace_highlighted_text(new_text: &str) -> Result<(), String>;
```

### `get_highlighted_text`

- **Signature**: `fn get_highlighted_text() -> Option<String>`
- **Behavior**:
  - Returns `Some(String)` when a backend successfully retrieves non-empty highlighted text from the current context.
  - Returns `None` when:
    - No backend is available or succeeds for the current platform.
    - No non-empty selection/clipboard text is found.
  - On unsupported OSes (neither Windows nor Linux), panics with a clear message.

### `replace_highlighted_text`

- **Signature**: `fn replace_highlighted_text(new_text: &str) -> Result<(), String>`
- **Behavior**:
  - Simulates typing `new_text` into the currently focused window, effectively replacing the current selection.
  - On **Linux**:
    - Returns `Ok(())` when `wtype` or `xdotool` is available and succeeds.
    - Returns `Err("wtype failed")` or `Err("xdotool failed")` when the respective tool is present but exits with a non-zero status.
    - Returns `Err("WSL typing not supported without GUI input tool (wtype/xdotool)")` when running under WSL without those tools.
    - Returns `Err("no typing tool available (wtype or xdotool)")` when neither tool is available.
  - On **Windows**:
    - Uses `SendInput` to send Unicode keystrokes and currently always returns `Ok(())` from the public function.
  - On unsupported OSes, panics with a clear message.

## Platform-specific notes

### Linux

- Reading highlighted text:
  - Prefers WSL clipboard via `powershell.exe` when running under WSL.
  - Falls through several Wayland and X11 tools, returning the first non-empty result.
- Replacing highlighted text:
  - Requires at least one of `wtype` or `xdotool` to be installed and available in `PATH`.
  - Under pure terminal-only or headless environments, replacement will typically return an error.

### Windows

- Uses COM and UI Automation via the `windows` crate with features:
  - `Win32_System_Com`
  - `Win32_UI_Accessibility`
  - `Win32_UI_Input_KeyboardAndMouse`
  - `Win32_Globalization`
- Attempts to:
  - Get the focused element and read its selected text via `IUIAutomationTextPattern`.
  - If that fails, fall back to `IUIAutomationValuePattern` to read the control value.
  - If still unsuccessful, walk the UI tree from the desktop root to find a control with selected text.
- For replacement, it sends keystrokes corresponding to the Unicode characters in `new_text`.

### Other OSes

- The crate is explicit about unsupported platforms:
  - Both public functions panic with clear messages.
  - This is verified by tests in the fallback module.


## License

This project is licensed under the **MIT License**. See the `LICENSE` file for details.

## Links

- Repository: <https://github.com/ArielSklare/highlight_lib>
- Documentation: <https://github.com/ArielSklare/highlight_lib/blob/main/README.md>


