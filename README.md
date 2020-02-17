# cabocha-rs

[![cabocha at crates.io](https://img.shields.io/crates/v/cabocha.svg)](https://crates.io/crates/cabocha)
[![cabocha at docs.rs](https://docs.rs/cabocha/badge.svg)](https://docs.rs/cabocha)
[![Actions Status](https://github.com/Yasu-umi/cabocha-rs/workflows/test/badge.svg)](https://github.com/Yasu-umi/cabocha-rs/actions)

## Example

```bash
cargo run --example test
```

## Usage

```toml
[dependencies]
cabocha = "*"
```

```Rust
use cabocha::parser::Parser;

fn main() {
  let parser = Parser::new("");
  let sentence = "我輩は猫である。";

  let mut tree = parser.parse_to_tree(sentence);

  println!("{}", tree.to_string(cabocha::consts::CABOCHA_FORMAT::TREE));
}
```

## LISENCE

[MIT](./LISENCE)
