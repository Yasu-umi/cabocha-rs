extern crate cabocha;

use cabocha::Parser;

fn main() {
    let mut parser = Parser::new("");

    let text1 = parser.parse_to_str("一郎は二郎が描いた絵を三郎に贈った。");
    println!("{}", text1);

    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    println!("{}", tree.to_string(cabocha::CABOCHA_FORMAT::CONLL));

    let tokens = tree.tokens();
    for opt_token in tokens {
        if opt_token.is_some() {
            let token = opt_token.unwrap();
            println!("{:?}", token);
        }
    }

    let chunks = tree.chunks();
    for opt_chunk in chunks {
        if opt_chunk.is_some() {
            let chunk = opt_chunk.unwrap();
            println!("{:?}", chunk);
        }
    }
}
