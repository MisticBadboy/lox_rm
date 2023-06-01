pub mod scanner;
pub mod token;
pub mod generateast;
pub mod parser;

use std::{env,process, fs, io::{self, Read, BufRead}};

static mut hadError : bool = false;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage jlox [script]");
        process::exit(64); 
    }
    else if args.len() == 2 {
        run_file(&args[1]);
    }
    else{
        run_prompt();
    }
}

fn error (line : i32, message : String) {
    report(line, ("").to_string(), message);
}

fn report(line : i32, _where : String, message: String){
    format!("[line{}] Error {}: {}",line,_where,message);
}

fn run_file(file: &str) {
    let content = fs::read_to_string(file).expect("Couldn't locate file");
    run(content);
    unsafe {
        if hadError == true { 
            process::exit(65); 
        }
    }
}

fn run(source: String) {
    let mut Scanner  = scanner::Scanner::new(source);
    let tokens = Scanner.scanTokens();
    let mut parser = parser::Parser::new(tokens.clone());
    let parser_expr = parser.expression();
    let string_expr = parser_expr.to_string();
    for token in tokens {
        println!("{:?}",token);
    }
    println!("{:?}", string_expr);
}

fn run_prompt(){
    let mut line;
    println!("Lox cmd lang please type anything");
    loop {
        print!("> ");
        line = String::new();
        let _result = match io::stdin().lock().read_line(&mut line) {
            Ok(_) => {
            if line == ("").to_string(){
                break;
            }
            run(line.to_owned());
            unsafe { hadError = false; } 
            },
            Err(e) => {
                println!("Error occurred : {}", e.to_string());
                process::exit(64);
            }
        };
        println!("")
    }
}