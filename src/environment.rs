use crate::expr::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines a variable in the environment.
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// Assigns a value to a variable in the environment.
    ///
    /// # Errors
    ///
    /// Returns an error if the variable is not defined.
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        let Some(v) = self.values.get_mut(name) else {
            return Err(format!("Undefined variable '{name}'."));
        };
        *v = value;
        Ok(())
    }

    /// Get the value of a variable in the environment.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}
