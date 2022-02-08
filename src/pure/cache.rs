use std::collections::HashMap;

use cache::*;

pub use cache::add_to_linker;

wit_bindgen_wasmtime::export!("./interfaces/cache.wit");

pub struct PureCache {
    contents: HashMap<String, Vec<u8>>
}

impl cache::Cache for PureCache {
    /// Set the payload for the given key.
    /// If provided, the time-to-live argument (in seconds) will be used to set the expiration time.
    fn set(&mut self, key: &str, value: PayloadParam<'_>, ttl: Option<u32>) -> Result<(), Error> {
        self.contents.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    /// Get the payload for the given key.
    fn get(&mut self, key: &str) -> Result<PayloadResult, Error> {
        match self.contents.get(&key.to_string()) {
            Some(result) => Ok(result.clone()),
            None => Err(Error::NotFoundError)
        }
    }

    /// Delete the entry for the given key.
    fn delete(&mut self, key: &str) -> Result<(), Error> {
        self.contents.remove(key);
        Ok(())
    }
}

impl PureCache {
    pub fn new() -> Self {
        PureCache {
            contents: HashMap::new()
        }
    }
}