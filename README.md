# nexish

**nexish** is a simple, experimental shell created as a learning project in Rust.

---

## 🚀 Overview

This project is designed primarily for learning and experimentation with:

- Rust's standard library (`std::fs`, `std::process`, etc.)
- Command-line argument parsing (manual and with crates like `clap`)
- File system interaction
- Basic shell design and command implementation

---

## ✨ Features

- Basic shell prompt and command loop
- Built-in commands: `ls`, `cd`, `pwd`, `mkdir`, `touch`, `rmdir`, and more
- Manual flag parsing (with plans for `clap` integration)
- Directory listing and navigation
- Minimalist and focused on understanding, not production

---

## 🛠️ Prerequisites

[![Rust](https://img.shields.io/badge/Requires-Rust-blue?logo=rust)](https://www.rust-lang.org/tools/install)

You need [Rust](https://www.rust-lang.org/tools/install) installed.

### 🖋️ Nerd Font Support (for Icons/Symbols)

Some parts of `nexish` use special Unicode symbols and icons (such as device logos) from [Nerd Fonts](https://www.nerdfonts.com/).  
**For the best experience**, configure your terminal to use a Nerd Font. Otherwise, some symbols may appear as squares or question marks.

- **How to use Nerd Fonts:**
  - Download a patched font from [Nerd Fonts](https://www.nerdfonts.com/font-downloads).
  - Set your terminal emulator to use this font (check your terminal's settings/preferences).
- **If you do not use a Nerd Font:** The shell will still work, but icons/logos may not display properly.

---

## 📦 Installation

You have two options: automated script or manual install.

### 🔹 1. Quick Install via Script

**Recommended:** Installs the latest release binary to your system PATH.

```bash
curl -sSfL https://raw.githubusercontent.com/santoshxshrestha/nexish/main/scripts/install.sh | bash
```

- This script will:
  1. Build `nexish` in release mode (if Rust is present).
  2. Copy the binary to `/usr/local/bin`.
  3. Make it executable.

> **Tip:** You may need to enter your password for `sudo` privileges.

---

### 🔹 2. Manual Build & Install

If you prefer full control or want to customize the build:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/santoshxshrestha/nexish.git
   cd nexish
   ```

2. **Build the Release Binary:**

   ```bash
   cargo build --release
   ```

   This places the binary at `target/release/nexish`.

3. **Copy to a PATH directory (e.g., `/usr/local/bin`):**

   ```bash
   sudo cp target/release/nexish /usr/local/bin/nexish
   ```

4. **(Optional) Ensure executable permission:**

   ```bash
   sudo chmod +x /usr/local/bin/nexish
   ```

5. **Run from anywhere:**

   ```bash
   nexish
   ```

---

## 🗑️ Uninstallation

You can uninstall using the provided script or manually:

### 🔹 1. Quick Uninstall via Script

```bash
curl -sSfL https://raw.githubusercontent.com/santoshxshrestha/nexish/main/scripts/uninstall.sh | bash
```

### 🔹 2. Manual Uninstall

Remove the binary from your PATH:

```bash
sudo rm /usr/local/bin/nexish
```

or

```bash
sudo rm /usr/bin/nexish
```

If you also want to remove your cloned repository:

```bash
rm -rf ~/nexish
```

---

## 🖥️ Usage

After installation, start the shell by typing:

```bash
nexish
```

You can use commands like:

```
ls
cd
pwd
ls -a
mkdir
touch
rmdir
```

---

## ⚠️ Notes

- **Learning project:** Not all edge cases are handled.
- Not intended for production use.
- Suggestions and contributions are welcome as part of the learning process.

---

## 📄 License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Made as a Rust learning exercise. Enjoy exploring!
