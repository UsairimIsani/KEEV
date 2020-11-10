use super::Store;
pub struct Storage {
    pub inner: BTreeMap<String, String>,
}

impl Store for Storage {
    fn get(&mut self, key: &str) -> anyhow::Result<&String> {
        self.innner
            .get(key)
            .ok_or(anyhow::Error::msg("No key found."))
    }
    fn set() {}
    fn remove() {}
    fn insert() {}
}
