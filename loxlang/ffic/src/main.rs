use std::process::exit;
use std::ffi::CString;

extern "C"{
    fn repl();
    fn initVM();
    fn freeVM();
    fn runFile(path:*const i8);
}


fn main() {
    let args:Vec<String> = std::env::args().collect();
    let argc = args.len();
    unsafe {
        initVM();

    if argc == 1{
        repl();
    }else if argc == 2{
        let c_string = CString::new(args[1].clone()).expect("CString::new failed");
        let c_ptr = c_string.as_ptr();
        runFile(c_ptr);
    }else {
        eprintln!("Usage: lox [path]");
        exit(64);
    }
        freeVM();
    }
}
