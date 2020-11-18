use super::disk::Disk;
use super::memory::Memory;
use super::Store;

pub struct DB {
    memory: Memory,
    disk: Disk,
}

impl Store for DB {
    fn new() -> DB {
        Self {
            memory: Memory::new(),
            disk: Disk::new(),
        }
    }
    fn get(&mut self, key: &str) -> anyhow::Result<&[u8]> {
        let r = self.memory.get(key)?;
        let _ = self.disk.get(key)?;
        Ok(r)
    }
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<&[u8]> {
        let r = self.memory.set(key, val)?;
        let _ = self.disk.set(key, val)?;
        Ok(r)
    }
    fn remove(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
        let _ = self.disk.remove(key)?;
        self.memory.remove(key)
    }
}
