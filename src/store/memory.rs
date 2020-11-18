use super::Store;
use crate::utils::serialize;
use std::collections::BTreeMap;
#[derive(Eq, PartialEq)]
pub struct Memory {
    pub inner: BTreeMap<String, Vec<u8>>,
}

impl Store for Memory {
    fn new() -> Memory {
        Self::log("new", "", "", "InMemory");
        Memory {
            inner: BTreeMap::new(),
        }
    }
    fn get(&mut self, key: &str) -> anyhow::Result<&[u8]> {
        Self::log("get", key, "", "InMemory");
        let r = self
            .inner
            .get(key)
            .ok_or(anyhow::Error::msg("No key found."))?;
        Ok(r.as_ref())
    }
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<&[u8]> {
        Self::log("set", key, val, "InMemory");
        self.inner
            .entry(key.to_string())
            .or_insert(serialize(key, val)?);
        self.get(key)
    }
    fn remove(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
        Self::log("remove", key, "", "InMemory");
        self.inner
            .remove(key)
            .ok_or(anyhow::Error::msg("No Key found"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert_key_val() {
        use super::*;
        let mut me = Memory {
            inner: BTreeMap::new(),
        };
        let mut b: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        b.insert(
            "hello".to_string(),
            serialize::serialize("hello", "world").unwrap(),
        );
        me.set("hello", "world").expect("Something is not Right!");
        assert_eq!(b, me.inner);
    }
    #[test]
    fn get_key() {
        use super::*;
        use crate::utils::deserialize::deserialize;
        let mut me = Memory {
            inner: BTreeMap::new(),
        };
        let key = "hello";
        let val = "world";
        me.set(key, val).expect("Something is not Right!");
        let data = me.get("hello").expect("Something is not Right!");
        let re = deserialize(data).unwrap();
        assert_eq!((key.to_string(), val.to_string()), re);
    }
    #[test]
    fn remove() {
        use super::*;
        use crate::utils::deserialize::deserialize;
        let mut me = Memory {
            inner: BTreeMap::new(),
        };
        let key = "hello";
        let val = "world";
        me.set(key, val).expect("Something is not Right!");
        let data = me.remove("hello").expect("Something is not Right!");
        let re = deserialize(&data).unwrap();
        assert_eq!((key.to_string(), val.to_string()), re);
    }
}
