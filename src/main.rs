mod datastore;
mod parser;
mod repl;
mod server;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about= None)]
struct Args {
    #[clap(subcommand)]
    mode: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Server { port: u16 },
    Repl,
}



fn main() {
    let args = Args::parse();
    match args.mode {
        Commands::Server { port } => {
            let options = server::ServerOptions::new(port);
            server::server(options);
        }
        Commands::Repl => {
            let datastore = datastore::DataStore::new();
            repl::repl(datastore);
        }
    }
    
}

fn self_standing_server() {
    let options = server::ServerOptions::new(8080);
    server::server(options);
}

fn self_standing_repl() {
    let datastore = datastore::DataStore::new();
    repl::repl(datastore);
}
