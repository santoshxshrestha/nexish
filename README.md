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

### Building

You need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
cargo build
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

## Notes

- This is a **learning project**. The code may not handle all edge cases or be ready for general use.
- Contributions or suggestions are welcome, but the main goal is personal improvement and exploration.

## License

MIT License

---

Made as a Rust learning exercise. Enjoy exploring!
