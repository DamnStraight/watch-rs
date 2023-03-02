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
    Add { path: Option<OsString> },
    Next,
    Upto { episode: usize }
}

// impl Run for Commands::Add {

// }