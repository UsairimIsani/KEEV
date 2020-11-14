pub trait Store {
    fn get(&mut self, key: &str) -> anyhow::Result<&Vec<u8>>;
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<()>;
    fn remove(&mut self, key: &str) -> Option<Vec<u8>>;
}
pub mod disk;
pub mod memory;
