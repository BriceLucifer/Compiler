use crate::token;

// 实现一个lexer
pub struct Lexer{
    input:Vec<char>, // 输入
    pub position:usize, // 位置
    pub read_position:usize, // 读取位置
    pub ch:char, // 字符
}

// 判断是否是字母
fn is_letter(ch:char) -> bool{
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

// 判断是否是数字
fn is_digital(ch:char) -> bool{
    '0' >= ch && ch <= '9'
}

impl Lexer {
    pub fn new(input: Vec<char>)-> Self{
        Self { 
            input: input, 
            position: 0, 
            read_position: 0, 
            ch: '0' 
        }
    }

    pub fn read_char(&mut self){
        if self.input.len() <= self.read_position{
            self.ch = '0';
        }else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self){
        let temp = self.ch;
        if temp == ' ' || temp == '\t' || temp == '\n' || temp == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        // 读取闭包辨别器
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        // 读取数字更新
        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digital(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        // 先定义tok
        let tok: token::Token;
        self.skip_whitespace();
        // 简易版本
        match self.ch {
            '=' => {
                tok = token::Token::ASSIGN(self.ch);
            },
            '+' => {
                tok = token::Token::PLUS(self.ch);
            },
            '-' => {
                tok = token::Token::MINUS(self.ch);
            },
            '0' => {
                tok = token::Token::EOF;
            }
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        Err(_err) => {
                            return token::Token::IDENT(ident);
                        }
                    }
                } else if is_digital(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return token::Token::INT(ident);
                } 
                else {
                    return token::Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }

}
