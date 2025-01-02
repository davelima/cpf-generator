# CPF Generator

Tired of using online test CPF generators and willing to learn Rust, I've written this tiny program that generates and
outputs a valid random CPF number.

## Building:
1. Ensure you have Rust/Cargo installed;
2. Build with `cargo build -r`;
3. Your binary will be available on `target/release/cpf-generator`. You can move it to `/usr/local/bin/` or any other directory.


## Usage:

```shell
# Outputs a valid random CPF:
./cpf-generator


# Add a valid random CPF to the clipboard (Wayland only)
./cpf-generator | wl-copy

# If you moved it to a directory that is in your $PATH (e.g /usr/local/bin), you can use the following instead:
cpf-generator
cpf-generator | wl-copy

```
