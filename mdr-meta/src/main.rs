use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, Parser, ValueEnum};
use libmdrmeta::Meta;
use multimap::MultiMap;
//use serde::{Deserialize, Serialize};
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
    /// Generate a full example file
    Example(ExampleArgs),

    /// Print metadata in JSON format
    ToJson(ToJsonArgs),

    /// Print metadata in TOML format
    ToToml(ToTomlArgs),

    /// Check metadata file for errors
    Check(CheckArgs),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FileFormat {
    Json,
    Toml,
}

impl ValueEnum for FileFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[FileFormat::Json, FileFormat::Toml]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            FileFormat::Json => PossibleValue::new("json"),
            FileFormat::Toml => PossibleValue::new("toml"),
        })
    }
}

#[derive(Debug, Parser)]
pub struct ExampleArgs {
    /// Output format
    #[arg(
        short,
        long,
        value_name = "FORMAT",
        default_value = "toml",
        value_parser(clap::value_parser!(FileFormat)),
    )]
    format: FileFormat,

    /// Output filename
    #[arg(short, long, value_name = "OUTPUT", default_value = "-")]
    outfile: String,
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
/// Check MDRepo metadata TOML
pub struct CheckArgs {
    /// Input filename
    #[arg(value_name = "FILE")]
    filename: String,

    /// JSON output
    #[arg(short, long)]
    json: bool,
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
        Some(Command::Example(args)) => {
            let mut out_file = open_outfile(&args.outfile)?;
            let meta = Meta::example();
            write!(
                out_file,
                "{}",
                if args.format == FileFormat::Json {
                    meta.to_json()?
                } else {
                    meta.to_toml()?
                }
            )?;
        }
        Some(Command::ToJson(args)) => {
            let mut out_file = open_outfile(&args.outfile)?;
            let meta = parse_file(&args.filename)?;
            write!(out_file, "{}", meta.to_json()?)?;
        }
        Some(Command::ToToml(args)) => {
            let mut out_file = open_outfile(&args.outfile)?;
            let meta = parse_file(&args.filename)?;
            write!(out_file, "{}", meta.to_toml()?)?;
        }
        Some(Command::Check(args)) => {
            let meta = parse_file(&args.filename)?;
            let errors = meta.find_errors();
            if errors.is_empty() {
                println!("No errors");
            } else if args.json {
                let mut json_errors = MultiMap::new();
                for (field, msg) in &errors {
                    json_errors.insert(field, msg)
                }
                println!("{}", serde_json::to_string_pretty(&json_errors).unwrap())
            } else {
                let num_errors = errors.len();
                println!(
                    "Found {num_errors} error{}:\n{}",
                    if num_errors == 1 { "" } else { "s" },
                    errors
                        .iter()
                        .map(|(fld, msg)| format!("{fld}: {msg}"))
                        .collect::<Vec<String>>()
                        .join("\n")
                );
            }
        }
        _ => unreachable!(),
    };

    Ok(())
}

// --------------------------------------------------
fn parse_file(filename: &str) -> Result<Meta> {
    Meta::from_file(filename).map_err(|e| anyhow!("{filename}: {e}"))
}

// --------------------------------------------------
fn open_outfile(filename: &str) -> Result<Box<dyn Write>> {
    match filename {
        "-" => Ok(Box::new(io::stdout())),
        out_name => Ok(Box::new(File::create(out_name)?)),
    }
}
