# has_command

Procedural macro for checking if a host system supports desired command.

## Requirements

- Rust ^1.30

## Usage

Add dependency to your `Cargo.toml`

```toml
[dependencies]
has_command = "*"
```

Add this to your code

```rust
use has_command::has_command;
```

or

```rust
#[macro_use]
extern_crate has_command;
```

Then you can use it as

```rust
use std::process::Command;

fn main() {
    run_ls();
}

#[has_command(ls)]
fn run_ls() {
    assert!(Command::new("ls")
        .output()
        .expect("ls should be supported")
        .status
        .success()
    );
}
```
