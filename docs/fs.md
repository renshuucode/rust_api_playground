# File System Module (`std::fs`)

This module provides functions to practice file system operations.

## Usage

Run the module with the following command:

```bash
cargo run -- --module fs --action <ACTION> --path <PATH>
```

## Examples

1. Read a file:

```bash
cargo run -- --module fs --action read --path test_files/hello.txt
```

2. Write to a file:

```bash
cargo run -- --module fs --action write --path test_files/output.txt
```

3. Get file metadata:

```bash
cargo run -- --module fs --action metadata --path test_files/hello.txt
```
