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
        // 死循环
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

    // 所以为啥read_position = 17 是因为
    // 在已经读取到末尾了 然后设置了 read_position = 15 => 又进行一步 l.read_char() read_position = 16 position = 15
    // 然而在末尾 tok = Token::EOF 之后又将 调用了一次 read_char() 所以导致 position = read_position (16)
    // read_position += 1 (17)
    println!("------------------");
    println!("input 长度{} \nlexer.ch = {}\nlexer.position = {}\nlexer.read+position = {}", input.len(),char::from(l.ch), l.position, l.read_position);
}