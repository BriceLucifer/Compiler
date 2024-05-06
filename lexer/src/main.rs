// lexer for text
mod token;
use token::*;
mod lexer;
use lexer::*;

fn main(){
    let input = String::from("let a = 1 + 3 ;");
    // 输入语句
    let mut l = lexer::Lexer::new(input.chars().collect());
    // 创建一个lexer 这样把input 绑定到lexer分析器

    l.read_char();
    // 读取第一个

    loop {
        let token = l.next_token();
        // 判断是不是next_token()

    // 如果next_token中里面 最后调用read_char()
    // input.len() <= read_position ? lexer.ch = '0' : lexer.ch = lexer.input(lexer.read+position)

        if token == token::Token::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }

    println!("{} {} {} {}", input.len(),char::from(l.ch), l.position, l.read_position);
}