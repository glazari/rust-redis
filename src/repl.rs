use crate::client::DataStoreClient;
use crate::datastore::DataStore;
use crate::datastore::DataStoreService;
use crate::parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn repl<T>(datastore: T) -> Result<()>
where
    T: DataStoreService,
{
    let mut rl = DefaultEditor::new()?;

    let mut counter = 0;
    loop {
        let readline = rl.readline(&format!("\x1b[1;32m[{}]:\x1b[0m ", counter));
        counter += 1;
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let mut parser = parser::Parser::new(line);
                let command = parser.parse();
                let result_str = match command {
                    Ok(command) => match datastore.execute(command) {
                        Ok(result) => match result {
                            Some(result) => result,
                            None => "(nil)".to_string(),
                        },
                        Err(msg) => format!("(error) {}", msg),
                    },
                    Err(msg) => format!("(error) {}", msg),
                };
                println!("{}", result_str.trim_end());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

pub fn stand_alone_repl() {
    let datastore = DataStore::new();
    repl(datastore);
}

pub fn repl_with_client(url: &str) {
    let datastore = DataStoreClient::new(url);
    repl(datastore);
}
