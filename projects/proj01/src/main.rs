use crate::expr_parser::tokenizer::Tokenizer;

mod expr_parser;

fn main() {
    let t = Tokenizer::new("hello");
    println!("Hello, world!");
}
