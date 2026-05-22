# findit

A fast, lightweight command-line filesystem search tool written in Rust. Search for directories, files, or words within files — with support for ignore files to filter out noise like `target` or `node_modules`.

---

## Usage

### Search for a directory
```
findit --dir <start_path> <name>
```
```
findit --dir . src
```

### Search for a file
```
findit --file <start_path> <name>
```
```
findit --file . main.rs
```

### Search for a word in a file
```
findit --word <file_path> <word>
```
```
findit --word ./src/main.rs fn
```

### Set ignore file
```
findit --ignore <file_path>
```
```
findit --ignore ~/.finditignore
```

### Verbose mode
Add `--verbose` to any search command to see the full traversal output.
```
findit --dir . src --verbose
```

---

## Ignore File

The ignore file works similarly to `.gitignore`. Each line is a pattern — any path containing that pattern will be skipped during traversal.

Example ignore file:
```
target
node_modules
.git
```

Set your ignore file once with `findit --ignore <path>` and it will persist across sessions.

---

## Installation

### Cargo
```
cargo install findit-rs
```

### curl
```
curl -sSL https://raw.githubusercontent.com/LukachuPro88/Findit/main/install.sh | sh
```

---

## License

[MIT](LICENSE.md) — Copyright 2026 Luka Saarivirta (LukachuPro88)
