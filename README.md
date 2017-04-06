# cabocha-rs

## Example
```toml
[dependencies]
cabocha = "*"
```
```Rust
extern crate cabocha;

use cabocha::Parser;

fn main() {
  let parser = Parser::new("");
  let sentence = "我輩は猫である。";

  let mut tree = parser.parse_to_tree(sentence);

  println!("{}", tree.to_string(cabocha::CABOCHA_FORMAT::TREE));
}
```