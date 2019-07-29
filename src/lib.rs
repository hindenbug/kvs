#[derive(Default)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self {
        KvStore {}
    }

    pub fn set(&mut self, key: String, value: String) -> String {
        panic!()
    }

    pub fn get(&self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&mut self, key: String) {
        panic!()
    }
}
