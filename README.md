
# ToDo CLI (Rust)

A tiny, beginner-friendly ToDo list for your terminal.

## Features
- Add / list / mark done / remove / clear
- Stores data in `~/.todo-cli.json`
- Zero setup besides Rust toolchain

## Usage
```bash
cargo run -- add "buy milk"
cargo run -- list
cargo run -- done 1
cargo run -- rm 1
cargo run -- clear
```

---

#Build
```bash
cargo build --release
./target/release/todo list
```

---

