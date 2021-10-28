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
    #[clap(about = "tree width", short, long, )]
    width: Option<u16>,

    #[clap(about = "raw | term | html | default", short, long, )]
    format: Option<String>,
}

pub enum Format {
    Raw,
    Term,
    Html,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "raw" => Ok(Format::Raw),
            "term" => Ok(Format::Term),
            "html" => Ok(Format::Html),
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
                Format::from_str(s)
            }
        }
    }

    pub fn width(&self) -> Result<u16, String> {
        let default_width: u16 = 25;
        let width: u16 = self.width
            .unwrap_or(default_width);
        let min_width = 17;
        if width < min_width {
            return Err(format!("width is too small, minimal value is {}", min_width));
        }
        if width % 2 == 0 {
            return Err("width should be odd".to_string());
        }
        let max_width = 59;
        if width > max_width {
            return Err(format!("width is too large, maximal value is {}", max_width));
        }
        Ok(width)
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
    Arguments { args: Opts::parse() }
}
