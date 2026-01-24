pub mod scanner;

use scanner::Scanner;

use std::io::{self, Write};

#[derive(Debug, Default)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Run a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn run_file(&mut self, path: String) -> std::io::Result<()> {
        self.run(&std::fs::read_to_string(path)?);
        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    /// Run a prompt.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be read.
    pub fn run_prompt(&mut self) -> std::io::Result<()> {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;

            input.clear();
            io::stdin().read_line(&mut input)?;

            let line = input.trim();
            if line.is_empty() {
                break;
            }
            self.run(line);
            self.had_error = false;
        }
        Ok(())
    }

    pub fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token:#?}");
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {line}] Error {location}: {message}");
        self.had_error = true;
    }
}
