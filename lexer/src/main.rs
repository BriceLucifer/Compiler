// lexer for text
mod token;
use token::Token;
mod lexer;
use lexer::*;

fn main(){
    let input = String::from("let a + - c b");
    let mut l = lexer::Lexer::new(input.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        if token == token::Token::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }
}