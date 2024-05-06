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
    '0' <= ch && ch <= '9'
}

impl Lexer {
    // 创建一个 lexer new函数
    pub fn new(input: Vec<char>)-> Self{
        Self { 
            input: input, 
            position: 0, 
            read_position: 0, 
            ch: '0' 
        }

        // 默认0 并且ch 为EOF
    }

    // 读取
    pub fn read_char(&mut self){
        // 输入和读取位置匹配 如果比读取位置小 那就EOF
        if self.input.len() <= self.read_position{
            self.ch = '0';
        }else {
            // 否则就迭代 读取 默认是self.read_position = 0
            self.ch = self.input[self.read_position];
        }

        // 循环位置为 之前读取的位置
        self.position = self.read_position;
        // 读取位置+1
        self.read_position += 1;
    }

    // 跳脱空白 其实就是 如果遇到空白 read_position += 1
    pub fn skip_whitespace(&mut self){
        let temp = self.ch;
        if temp == ' ' || temp == '\t' || temp == '\n' || temp == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        
        // 读取字符辨别器
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };
        /*
            其实就是获取lexer 然后读取位置 如果现在的位置 并且比input的长度
            短 并且是字符 下一个读取
            如果位置和l.input.len() == position
            那就跳脱 然后返回剩下的vec切片
        */

        // 读取数字更新
        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digital(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        // 先定义token 然后匹配 一定是mut 相当于一个暂时的空间变量
        let tok:token::Token;
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
            '!' => {
                tok = token::Token::BANG(self.ch);
            },
            '/' => {
                tok = token::Token::SLASH(self.ch);
            },
            '*' => {
                tok = token::Token::ASTERISK(self.ch);
            },
            '<' => {
                tok = token::Token::LT(self.ch);
            },
            '>' => {
                tok = token::Token::GT(self.ch);
            },
            ';' => {
                tok = token::Token::SEMICOLON(self.ch);
            },
            '(' => {
                tok = token::Token::LPAREN(self.ch);
            },
            ')' => {
                tok = token::Token::RPAREN(self.ch);
            },
            ',' => {
                tok = token::Token::COMMA(self.ch);
            },
            '{' => {
                tok = token::Token::LBRACE(self.ch);
            },
            '}' => {
                tok = token::Token::RBRACE(self.ch);
            },
            '0' => {
                tok = token::Token::EOF;
            }
            _ => {
                // 如果开头就是letter 那么我就调用read_identifier
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    // 获取关键字的字数
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        // 如果不是关键字中的一员 那就是变量了
                        Err(_err) => {
                            return token::Token::IDENT(ident);
                        }
                    }
                } else if is_digital(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return token::Token::INT(ident);
                    // 如果是数字返回是INt类型
                } 
                else {
                    // 否则就是非法
                    return token::Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }

}
