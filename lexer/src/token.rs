#[derive(Debug,PartialEq)]
pub enum Token {
    // 关键字 key world
    ILLEGAL,
    EOF,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    FUNCTION,

    // 变量
    IDENT(Vec<char>),
    INT(Vec<char>),

    // 操作符号
    ASSIGN(char), // =
    PLUS(char),  // +
    MINUS(char), // -
    BANG(char), // !
    ASTERISK(char), // *
    LT(char),GT(char),  // <小于  //  >大于

    // 标点符号
    COMMA(char),  // ,
    SEMICOLON(char),  // ;
    LPAREN(char), // (
    RPAREN(char), // )
    LBRACE(char), // {
    RBRACE(char), // }
    SLASH(char), // \
}

pub fn get_keyword_token(ident : &Vec<char>)-> Result<Token,String>{
    
    // 将鉴别器粉碎 然后collect 就是vec
    let identifer :String = ident.into_iter().collect();
    match &identifer[..] {
        // 关键字匹配
        "let" => Ok(Token::LET),
        "fn" => Ok(Token::FUNCTION),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        
        _ => Err(String::from("Not a key world")),
    }
}