use std::{env, error::Error};

use crate::cmd::{Next, Run};
use crate::db;

impl Run for Next {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = db::Database::init(env::current_dir()?)?;

        db.watch_next()?;

        Ok(())
    }
}
