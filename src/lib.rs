mod metatoml;
mod types;

use crate::metatoml::Meta;
use anyhow::Result;
use clap::Parser;

// --------------------------------------------------
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Create sufr file
    Validate(ValidateArgs),
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Validate MDRepo metadata TOML
pub struct ValidateArgs {
    /// Input filename
    #[arg(value_name = "FILE")]
    filename: String,
}

// --------------------------------------------------
pub fn validate(args: &ValidateArgs) -> Result<()> {
    let meta = Meta::from_file(&args.filename)?;
    dbg!(&meta);
    let errors = &meta.find_errors();
    if !errors.is_empty() {
        eprintln!("{}", errors.join("\n"));
    }

    Ok(())
}
