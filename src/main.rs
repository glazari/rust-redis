mod datastore;
mod parser;
mod repl;
mod server;

fn main() {
    let options = server::ServerOptions::new(8080);
    server::server(options);
}

fn self_standing_repl() {
    let datastore = datastore::DataStore::new();
    repl::repl(datastore);
}
