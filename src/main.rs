#![allow(clippy::comparison_chain)]

use rlox::Lox;

fn main() -> std::io::Result<()> {
    let mut lox = Lox::new();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(args[1].clone())?;
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}
