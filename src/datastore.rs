use std::collections::HashMap;
use std::sync::Mutex;

pub trait DataStoreService {
    fn execute(&self, command: Command) -> Option<String>;
}

pub struct DataStore {
    data: Mutex<HashMap<String, String>>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: &str, value: &str) -> Option<String> {
        self.data
            .lock()
            .unwrap()
            .insert(key.to_string(), value.to_string())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).map(|s| s.to_string())
    }
}

impl DataStoreService for DataStore {
    fn execute(&self, command: Command) -> Option<String> {
        match command {
            Command::Set { key, value } => self.set(&key, &value),
            Command::Get { key } => self.get(&key).map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::Set { key, value } => format!("set {} {}", key, value),
            Command::Get { key } => format!("get {}", key),
        }
    }

    #[cfg(test)]
    pub fn set(key: &str, value: &str) -> Command {
        Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    #[cfg(test)]
    pub fn get(key: &str) -> Command {
        Command::Get {
            key: key.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set() {
        let datastore = DataStore::new();
        datastore.set("foo", "bar");
        assert_eq!(datastore.get("foo"), Some("bar".to_string()));
    }

    #[test]
    fn test_get_non_existent() {
        let datastore = DataStore::new();
        assert_eq!(datastore.get("foo"), None);
    }
}
