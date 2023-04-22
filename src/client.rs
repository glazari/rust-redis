use crate::datastore::Command;
use crate::datastore::DataStoreService;

pub struct DataStoreClient {
    url: String,
}

impl DataStoreClient {
    pub fn new(url: &str) -> DataStoreClient {
        DataStoreClient {
            url: url.to_string(),
        }
    }
}

impl DataStoreService for DataStoreClient {
    fn execute(&self, command: Command) -> Result<Option<String>, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&self.url)
            .body(command.to_string())
            .send().map_err(|e| e.to_string())?;
            
        let body = response.text().unwrap();
        Ok(Some(body))
    }
}
