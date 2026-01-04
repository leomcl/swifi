# Swifi

**Swifi** is a lightweight CLI tool to check wifi speeds.

Faster __(pending benchmarking)__, easier, and cooler than web alternatives.

## Quick Start

### Option 1 - single command 

If you have Rust installed I think you can run

```
cargo install --git https://github.com/leomcl/swifi
```

### Option 2 - from source

1. Clone repo
```
git clone https://github.com/leomcl/swifi.git
cd swifi
```

2. Build and install
```
cargo install --path .
```

## Usage

```
swifi --help 
```

## Development

## Future work

- TUI
- Benchmarks
- Better API
- Replacing speed test (defunct)

### Testing

Run all tests:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run integration tests only:
```bash
cargo test --test cli_flags
```

### Linting

Check for issues:
```bash
cargo clippy --all-targets --all-features
```

**Note:** that we have used the following vscode settings.json which follow the Linux 
kernel style guide of 8 spaces per tab key. 

```json
{
        "editor.tabSize": 8,
        "editor.insertSpaces": true,
        "editor.detectIndentation": false,
        "editor.rulers": [80],
        "editor.wordWrapColumn": 80,
        "[rust]": {
                "editor.defaultFormatter": "rust-lang.rust-analyzer",
                "editor.formatOnSave": true,
                "editor.tabSize": 8,
                "editor.insertSpaces": true,
                "editor.detectIndentation": false,
                "editor.rulers": [80]
        }
}
```