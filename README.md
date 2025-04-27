# Word Counter

A simple command-line utility written in Rust that counts words in text files, with proper support for CJK characters.

## Features

- Properly counts words in text containing CJK characters, ignoring punctuation marks.
- Supports reading from files or stdin (pipe)
- Can process multiple files at once

## Installation

```bash
cargo install cwc
```

## Usage

### Count words in a file:

```bash
cwc filename.txt
```

### Count words in multiple files:

```bash
cwc file1.txt file2.txt file3.txt
```

### Count words from stdin (pipe):

```bash
cat file.txt | cwc
```
OR

```bash
echo "Some text to count" | cwc
```

### Display version:

```bash
cwc -v
```
