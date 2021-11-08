use std::io::Write;

use scanner::Scanner;
use token::Token;

mod scanner;
mod token;
mod token_type;
fn main() {
    let runner = RloxRunner::new();
    match std::env::args().len() {
        // first argument in args is the process name so we omit including it
        1 => runner.run_prompt(),
        2 => runner.run_file(&std::env::args().nth(1).expect("No filename passed to rlox")),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    }
}

struct RloxRunner {
    had_error: bool,
}

impl RloxRunner {
    pub fn new() -> RloxRunner {
        RloxRunner { had_error: false }
    }

    pub fn run_prompt(&self) -> () {
        let mut line = String::new();
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
                    line = line.trim().to_string();
                    if line.len() == 0 {
                        break;
                    };
                    self.run(&line)
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }

    pub fn run_file(self, file: &String) -> () {
        let contents = std::fs::read_to_string(file);
        match contents {
            Ok(src) => self.run(&src),
            _err => panic!("Received invalid filename {}", file),
        }
    }

    pub fn run(&self, source: &String) -> () {
        let mut scanner = Scanner::new(source.chars().collect());
        let mut tokens: Vec<Token> = Vec::new();
        scanner.scan_tokens(&mut tokens);

        for token in tokens {
            println!("{}", token)
        }
    }

    fn error(&mut self, line: i32, msg: String) {
        self.report(line, "", msg)
    }

    fn report(&mut self, line: i32, location: &str, msg: String) {
        println!("[line {}], Error{}: {}", line, location, msg);
        self.had_error = true
    }
}
