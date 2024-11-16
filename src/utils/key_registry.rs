use std::collections::HashMap;

// Utility structure to generate "public keys" and associate them to a name, to easily print the amount of money everyone has.
pub struct KeyRegistry {
    keys: HashMap<u32, String>,
    next_key: u32,
}

impl KeyRegistry {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            next_key: 1,
        }
    }

    pub fn generate(&mut self, name: &str) -> u32 {
        let key = self.next_key;

        self.next_key += 1;
        self.keys.insert(key, name.to_string());

        key
    }

    pub fn names(&self) -> &HashMap<u32, String> {
        &self.keys
    }
}
