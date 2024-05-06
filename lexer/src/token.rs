#[derive(Debug,PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    PLUS(char),
    MINUS(char),
    ASSIGN(char),
    LET,
    IDENT(Vec<char>),
    INT(Vec<char>),
}

pub fn get_keyword_token(ident : &Vec<char>)-> Result<Token,String>{
    let mut identifer :String = ident.into_iter().collect();
    match &identifer[..] {
        "let" => Ok(Token::LET),
        _ => Err(String::from("Not a key world")),
    }
}