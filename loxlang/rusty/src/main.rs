mod chunk;
mod common;
mod debug;
mod vm;
mod value;
mod object;
mod table;
mod compiler;

use std::process::exit;
use vm::{initVM,freeVM};

#[warn(dead_code)]

fn repl(){
    loop {
        println!("repl");
    }
}

fn read_file(path:&str)->Result<String,()>{
    println!("read file {}",path);
    Err(())
}

fn run_file(path:&str){
    println!("run file {}",path);
}

fn main(){
    initVM();

    let args:Vec<String> = std::env::args().collect();
    let argc = args.len();

    if argc == 1{
        repl();
    }else if argc == 2 {
        run_file(&args[1]);
    }else {
        eprintln!("Usage: rusty [path]\n");
        exit(64);
    }

    freeVM();
}
