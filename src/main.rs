use anyhow::Result;
use clap::Parser;
use mdrtoml::{Cli, Command};

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Cli) -> Result<()> {
    match &args.command {
        Some(Command::Validate(args)) => mdrtoml::validate(args)?,
        _ => unreachable!(),
    };

    Ok(())
}
