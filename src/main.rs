mod cmd;
mod db;
mod util;

use clap::Parser;
use cmd::Run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cmd::Cli::parse().run()?;

    Ok(())
}
