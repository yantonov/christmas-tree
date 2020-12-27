use clap::{Clap, crate_version};
use std::str::FromStr;

#[derive(Clap)]
#[clap(version = crate_version ! ())]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
pub enum Command {
    #[clap(about = "calculate elapsed time since given date", display_order = 0)]
    Show(Show)
}

#[derive(Clap)]
pub struct Show {
    #[clap(about = "raw | term | default", short, long, )]
    format: Option<String>
}

pub enum Format {
    Raw,
    Term,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "raw" => Ok(Format::Raw),
            "term" => Ok(Format::Term),
            "default" => Ok(Format::Term),
            _ => Err(format!("invalid format: {}", value))
        }
    }
}

impl Show {
    pub fn format(&self) -> Result<Format, String> {
        match &self.format {
            None => Ok(Format::Term),
            Some(s) => {
                Format::from_str(&s)
            }
        }
    }
}

pub struct Arguments {
    args: Opts
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }
}

pub fn arguments() -> Arguments {
    return Arguments { args: Opts::parse() };
}
