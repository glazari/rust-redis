use std::io::Write;
use crate::parser;

pub fn repl() {
    let mut counter = 0;
    loop {
        print!("[{}]: ", counter);
        // flush to stdout
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parser = parser::Parser::new(input);
        let command = parser.parse();
        println!("Parsed: {:?}", command);
        counter += 1;
    }
}
