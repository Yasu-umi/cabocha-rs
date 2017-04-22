extern crate cabocha;

use cabocha::Parser;

fn main() {
    let parser = Parser::new("");

    let text1 = parser.parse_to_str("一郎は二郎が描いた絵を三郎に贈った。");
    println!("{}", text1);

    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    println!("{}", tree.to_string(cabocha::CABOCHA_FORMAT::XML));

    let tokens = tree.tokens();
    for opt_token in tokens {
        if opt_token.is_some() {
            let token = opt_token.unwrap();
            println!("surface: {}", token.surface);
            println!(" normalized_surface: {}", token.normalized_surface);
            println!(" feature: {}", token.feature);
            println!(" feature_list: {:?}", token.feature_list);
            println!(" feature_list_size: {}", token.feature_list_size);
            println!(" ne: {}", token.ne);
            println!(" additional_info: {}", token.additional_info);
        }
    }

    let chunks = tree.chunks();
    for opt_chunk in chunks {
        if opt_chunk.is_some() {
            let chunk = opt_chunk.unwrap();
            println!("link: {}", chunk.link);
            println!(" head_pos: {}", chunk.head_pos);
            println!(" func_pos: {}", chunk.func_pos);
            println!(" token_size: {}", chunk.token_size);
            println!(" token_pos: {}", chunk.token_pos);
            println!(" score: {}", chunk.score);
            println!(" feature_list: {:?}", chunk.feature_list);
            println!(" additional_info: {}", chunk.additional_info);
            println!(" feature_list_size: {}", chunk.feature_list_size);
        }
    }
}
