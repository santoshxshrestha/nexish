# nexsh

**nexsh** is a simple, experimental shell created as a learning project in Rust.

## Purpose

This project is made for personal learning and to improve understanding of:

- Rust's standard library (`std::fs`, `std::process`, etc)
- Command-line argument parsing (manual and with crates like `clap`)
- File system interaction
- Basic shell design and command implementation

## Features

- Basic shell prompt and command loop
- Built-in commands: `ls`, `cd`, `pwd`, and more
- Manual flag parsing (with optional plans to use `clap`)
- Listing directory contents and changing directories
- Focus on understanding, not production use

## Usage

You need [Rust](https://www.rust-lang.org/tools/install) installed.

### Building (Debug build)

```bash
cargo build
```

### Building (Release build)

```bash
cargo build --release
```

### Running

```bash
cargo run
```

This will start the shell. You can type commands like:

```
ls
cd
cd ..
pwd
ls -a
mkdir
touch
rmdir
```

---

## Installing `nexsh` system-wide

If you want to use `nexsh` from any directory in your terminal, you can copy the compiled binary into a directory included in your system's `PATH`, such as `/usr/local/bin` (recommended) or `/usr/bin`.

### Steps

1. **Build the Release Binary**

   ```bash
   cargo build --release
   ```

   This will place the binary at `target/release/nexsh`.

2. **Copy to `/usr/local/bin` (recommended)**

   ```bash
   sudo cp target/release/nexsh /usr/local/bin/nexsh
   ```

   Or, to `/usr/bin` (if you prefer):

   ```bash
   sudo cp target/release/nexsh /usr/bin/nexsh
   ```

3. **(Optional) Ensure it is executable**

   Usually, Cargo makes it executable, but you can be sure by running:

   ```bash
   sudo chmod +x /usr/local/bin/nexsh
   ```

4. **Run `nexsh` from anywhere**

   Now you can just type:

   ```bash
   nexsh
   ```

   from any terminal prompt.

#### Uninstallation

To remove `nexsh` from your system:

```bash
sudo rm /usr/local/bin/nexsh
```

or

```bash
sudo rm /usr/bin/nexsh
```

---

## Notes

- This is a **learning project**. The code may not handle all edge cases or be ready for general use.
- Contributions or suggestions are welcome, but the main goal is personal improvement and exploration.

## License

MIT License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)---

Made as a Rust learning exercise. Enjoy exploring!
