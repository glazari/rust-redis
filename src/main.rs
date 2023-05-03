mod datastore;
mod parser;
mod repl;
mod server;
mod client;
mod loadtester;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about= None)]
struct Args {
    #[clap(subcommand)]
    mode: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Server { 
        #[clap(short, long, default_value = "8080")]
        port: u16 
    },
    Repl,
    Client { 
        #[clap(short, long, default_value = "http://localhost:8080")]
        url: String 
    },
    Loadtester { 
        #[clap(short, long, default_value = "http://localhost:8080")]
        url: String,

        #[clap(short, long, default_value = "100")]
        num_requests: usize,
    },
}

fn main() {
    let args = Args::parse();
    match args.mode {
        Commands::Server { port } => {
            let options = server::ServerOptions::new(port);
            server::server(options);
        }
        Commands::Repl => repl::stand_alone_repl(),
        Commands::Client { url } => repl::repl_with_client(&url),
        Commands::Loadtester { url, num_requests } => loadtester::loadtest(&url, num_requests),
    }
}
