# minigrep

[![Rust](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/claudedjoumessi/minigrep/ci.yml?style=for-the-badge&logo=github)](https://github.com/claudedjoumessi/minigrep/actions)
[![Crates.io](https://img.shields.io/crates/v/minigrep?style=for-the-badge)](https://crates.io/crates/minigrep)

**minigrep** is a blazing fast ⚡, minimal, and effective command-line tool for searching text patterns in files, inspired by the classic `grep` utility. Built with Rust for performance and reliability, minigrep is perfect for developers, sysadmins, and anyone who needs quick and accurate text searching.

## ✨ Features

- 🚀 **Fast and lightweight**: Written in Rust for speed and low resource usage.
- 🔍 **Simple usage**: Intuitive command-line interface.
- 📂 **Search multiple files**: Recursively search directories.
- 🛠️ **Customizable**: Easily extendable for your needs.

## 📦 Installation

From Source

```sh
git clone https://github.com/yourusername/minigrep.git
cd minigrep
cargo build --release
```

## ⚡ Usage

```sh
minigrep <pattern> <file>
```

Example:

```sh
minigrep "fn main" src/main.rs
```

## 🤔 Why minigrep?

- **✅ Performance**: Outpaces traditional tools for many use cases.
- **✅ Simplicity**: No unnecessary features—just what you need.
- **✅ Reliability**: Safe and robust, thanks to Rust.

## 🤝 Contributing

We welcome contributions from the community 🎉! Whether it's bug reports, feature suggestions, or pull requests, your input helps make minigrep better for everyone.

---

**💡 Try minigrep today and experience efficient searching!**
