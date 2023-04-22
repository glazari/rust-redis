use std::io::Write;
use crate::parser;
use crate::datastore::DataStore;
use crate::datastore::Command;

pub fn repl(mut datastore:  DataStore) {
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
        match command {
            Command::Set { key, value } => {
                datastore.set(&key, &value);
            }
            Command::Get { key } => {
                match datastore.get(&key) {
                    Some(value) => println!("{}: {}", key, value),
                    None => println!("{} not found", key),
                }
            }
        }
        counter += 1;
    }
}
