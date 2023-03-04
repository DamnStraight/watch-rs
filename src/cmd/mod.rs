mod add;
mod next;
mod upto;

use clap::{Parser, Subcommand};
use std::error::Error;
use std::ffi::OsString;

pub trait Run {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Parser)]
#[command(name = "watch")]
#[command(about = "Local media tracker CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(Add),
    Next(Next),
    Upto(Upto),
}

#[derive(Debug, Parser)]
pub struct Add {
    path: Option<OsString>,
}

#[derive(Debug, Parser)]
pub struct Next;

#[derive(Debug, Parser)]
pub struct Upto {
    episode: usize,
}

impl Run for Cli {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Commands::Add(cmd) => cmd.run(),
            Commands::Next(cmd) => cmd.run(),
            Commands::Upto(cmd) => cmd.run(),
        }
    }
}
