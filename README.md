# 🦀 Rust Projects Playground

A growing collection of Rust mini-projects and code experiments I’m building as I sharpen my skills and explore the language in depth. Most exercises are inspired by topics in the Brown CS online edition of *The Rust Programming Language* (<https://rust-book.cs.brown.edu/>).

---

## Projects

| Folder | Focus |
|--------|-------|
| `ownership_notes/` | precise ownership & borrowing demos |
| `mini_blog/` | structs, methods, and basic CLI CRUD |
| `cli_args/` | enums, pattern matching, and sub-command parsing |
| `word_freq/` | collections, strings, and iterators |
| `file_fetcher/` | robust error handling with `Result` + `anyhow` |
| `cache/` | generics and trait-driven LRU cache |
| `pool/` | lifetime-safe connection-pool sketch |
| More coming… | advanced concurrency, async, and performance tuning |

*(New projects will land here as time allows.)*

---

## Getting Started

```bash
# install stable toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# run any project
git clone https://github.com/<your-handle>/rust-projects.git
cd rust-projects/mini_blog
cargo run