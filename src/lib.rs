use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.store.get(&key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
