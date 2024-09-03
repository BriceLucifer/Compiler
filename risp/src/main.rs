mod object;
mod lexer;
mod parser;
mod eval;
mod repl;
mod env;

fn main() {
    println!("Hello, world!");
    let temp = "(
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )";
    let result = lexer::tokenize(temp);
    println!("{:#?}",result.unwrap())

}
