use std::collections::HashMap;

use crate::expr::Value;

#[derive(Debug, Default)]
pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}
