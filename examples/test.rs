extern crate cabocha;

use cabocha::Parser;

fn main() {
    let mut parser = Parser::new("");

    let text1 = parser.parse_to_str("一郎は二郎が描いた絵を三郎に贈った。");
    println!("{}", text1);

    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    println!("{}", tree.to_string(cabocha::CABOCHA_FORMAT::CONLL));

    let tokens = tree.token_iter();
    for token in tokens {
        println!("{:?}", token);
    }

    let chunks = tree.chunk_iter();
    for chunk in chunks {
        println!("{:?}", chunk);
    }
}
