mod parser;
mod repl;
mod datastore;

fn main() {
    let datastore = datastore::DataStore::new();
    repl::repl(datastore);
}
