#![allow(clippy::comparison_chain)]

use clap::Parser;
use rlox::Lox;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run a script file
    script: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut lox = Lox::new();
    match args.script {
        None => lox.run_prompt(),
        Some(script) => lox.run_file(script),
    }
}
