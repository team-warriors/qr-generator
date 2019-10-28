# Qr-Generator

[![GitHub issues](https://img.shields.io/github/issues/adiatma/qr-generator)](https://github.com/adiatma/qr-generator/issues)
[![GitHub forks](https://img.shields.io/github/forks/adiatma/qr-generator)](https://github.com/adiatma/qr-generator/network)
[![GitHub stars](https://img.shields.io/github/stars/adiatma/qr-generator)](https://github.com/adiatma/qr-generator/stargazers)

## Install

Please install rust with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`, and than if success install, please check with type `cargo version`.

```
[dependencies]
qr-generator = "0.1.0"
```

## How to use?

```rust
extern crate qr_generator;

fn main() {
  let content = "https://example.com/path".to_string();
  let name = "name_of_image".to_string();

  qr_generator::generate(content, name).unwrap(); // to generate qr-code to image.
}
```

## License

MIT
