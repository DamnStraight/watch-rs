use std::{env, error::Error};

use crate::cmd::{Run, Upto};
use crate::db;

impl Run for Upto {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = db::Database::init(env::current_dir()?)?;

        db.watch_up_to(self.episode)?;

        Ok(())
    }
}
