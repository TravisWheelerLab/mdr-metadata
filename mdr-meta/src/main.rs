use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, Write},
};

// --------------------------------------------------
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Print metadata in JSON format
    ToJson(ToJsonArgs),

    /// Print metadata in JSON format
    ToToml(ToTomlArgs),

    /// Validate metadata file
    Validate(ValidateArgs),
}

#[derive(Debug, Parser)]
pub struct ToJsonArgs {
    /// Input filename
    #[arg(value_name = "FILE")]
    filename: String,

    /// Output filename
    #[arg(short, long, value_name = "OUTPUT", default_value = "-")]
    outfile: String,
}

#[derive(Debug, Parser)]
pub struct ToTomlArgs {
    /// Input filename
    #[arg(value_name = "FILE")]
    filename: String,

    /// Output filename
    #[arg(short, long, value_name = "OUTPUT", default_value = "-")]
    outfile: String,
}

#[derive(Debug, Parser)]
/// Validate MDRepo metadata TOML
pub struct ValidateArgs {
    /// Input filename
    #[arg(value_name = "FILE")]
    filename: String,
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Cli) -> Result<()> {
    match &args.command {
        Some(Command::ToJson(args)) => {
            //let mut out_file: Box<dyn Write> = match args.outfile.as_str() {
            //    "-" => Box::new(io::stdout()),
            //    out_name => Box::new(File::create(out_name)?),
            //};
            let mut out_file = open_outfile(&args.outfile)?;
            let json = libmdrmeta::to_json(&args.filename)?;
            writeln!(out_file, "{json}")?;
        }
        Some(Command::ToToml(args)) => {
            let mut out_file = open_outfile(&args.outfile)?;
            let toml = libmdrmeta::to_toml(&args.filename)?;
            writeln!(out_file, "{toml}")?;
        }
        Some(Command::Validate(args)) => libmdrmeta::validate(&args.filename)?,
        _ => unreachable!(),
    };

    Ok(())
}

// --------------------------------------------------
fn open_outfile(filename: &str) -> Result<Box<dyn Write>> {
    match filename {
        "-" => Ok(Box::new(io::stdout())),
        out_name => Ok(Box::new(File::create(out_name)?)),
    }
}
