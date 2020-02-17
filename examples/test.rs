use cabocha::parser::Parser;

fn main() {
  let mut parser = Parser::new("");
  let text = "一郎は二郎が描いた絵を三郎に贈った。";

  println!("{}", parser.parse_to_str(text));

  let tree = parser.parse_to_tree(text);
  println!(
    "{}",
    tree.to_string(cabocha::consts::CABOCHA_FORMAT::LATTICE)
  );

  for (idx, chunk) in tree.chunk_iter().enumerate() {
    println!(
      "* {} {}D {}/{} {}",
      idx,
      chunk.link(),
      chunk.head_pos(),
      chunk.func_pos(),
      chunk.score(),
    );
    let token_pos = chunk.token_pos();
    for i in token_pos..(token_pos + chunk.token_size()) {
      if let Some(token) = tree.token(i) {
        println!("{}\t{}", token.surface(), token.feature());
      }
    }
  }
}
