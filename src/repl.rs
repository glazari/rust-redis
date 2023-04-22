use crate::datastore::DataStore;
use crate::client::DataStoreClient;
use crate::datastore::DataStoreService;
use crate::parser;
use std::io::Write;

fn repl<T>(datastore: T)
where
    T: DataStoreService,
{
    let mut counter = 0;
    loop {
        // color the prompt
        print!("\x1b[1;32m");
        print!("[{}]: ", counter);
        print!("\x1b[0m");
        // flush to stdout
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parser = parser::Parser::new(input);
        let command = parser.parse();
        //println!("Parsed: {:?}", command);
        let result_str = match command {
            Ok(command) => match datastore.execute(command) {
                Ok(result) => match result {
                    Some(result) => result,
                    None => "(nil)".to_string(),
                },
                Err(msg) => format!("(error) {}", msg),
            }
            Err(msg) => format!("(error) {}",msg),
        };
        println!("{}", result_str.trim_end());
        counter += 1;
    }
}

pub fn stand_alone_repl() {
    let datastore = DataStore::new();
    repl(datastore);
}

pub fn repl_with_client(url: &str) {
    let datastore = DataStoreClient::new(url);
    repl(datastore);
}
