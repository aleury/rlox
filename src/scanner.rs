pub struct Scanner {}

impl Scanner {
    #[must_use]
    pub fn new(_source: &str) -> Self {
        Self {}
    }

    #[must_use]
    pub fn scan_tokens(&self) -> Vec<Token> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Token {}
