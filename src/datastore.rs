use std::collections::HashMap;

pub struct DataStore {
    data: HashMap<String, String>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.data.insert(key.to_string(), value.to_string())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Set { key, value } => format!("set {} {}", key, value),
            Command::Get { key } => format!("get {}", key),
        }
    }

    pub fn Set(key: &str, value: &str) -> Command {
        Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn Get(key: &str) -> Command {
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
        let mut datastore = DataStore::new();
        datastore.set("foo", "bar");
        assert_eq!(datastore.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_get_non_existent() {
        let datastore = DataStore::new();
        assert_eq!(datastore.get("foo"), None);
    }
}
