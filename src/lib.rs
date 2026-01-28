pub mod ast_printer;
pub mod environment;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;

use scanner::Scanner;
use std::io::Write;

use crate::{
    interpreter::Interpreter,
    parser::{ParseError, Parser},
    scanner::ScanError,
};

#[derive(Debug, Default)]
pub struct Lox {
    pub had_error: bool,
    pub had_runtime_error: bool,
    interpreter: Interpreter,
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
        if self.had_runtime_error {
            std::process::exit(70);
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
            std::io::stdout().flush()?;

            input.clear();
            std::io::stdin().read_line(&mut input)?;

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
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        if !scanner.errors.is_empty() {
            self.report_scan_errors(scanner.errors);
            return;
        }
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(statements) => {
                if let Err(err) = self.interpreter.interpret(statements) {
                    self.had_runtime_error = true;
                    self.error(err.token.line, &err.message);
                }
            }
            Err(ParseError::UnexpectedToken { token, message }) => {
                self.had_error = true;
                self.error(
                    token.line,
                    &format!("Unexpected Token: {}, {message}", token.lexeme),
                );
            }
        }
    }

    fn report_scan_errors(&mut self, errors: Vec<ScanError>) {
        for error in errors {
            match error {
                ScanError::IllegalCharacter { line, character } => {
                    self.error(line, &format!("Illegal character '{character}'"));
                }
                ScanError::UnterminatedString { line } => {
                    self.error(line, "Unterminated string");
                }
                ScanError::ParseNumberError { lexeme, line } => {
                    self.error(line, &format!("Could not parse number '{lexeme}'"));
                }
            }
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
