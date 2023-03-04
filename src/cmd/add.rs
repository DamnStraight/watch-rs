use std::{env, error::Error, path::Path};

use crate::cmd::{Add, Run};
use crate::db;
use crate::util;

impl Run for Add {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let path = self
            .path
            .as_deref()
            .map(|s| s.to_str().unwrap().to_string())
            .unwrap_or(env::current_dir().unwrap().to_str().unwrap().to_string());

        let maybe_path = Path::new(&path);

        if !maybe_path.exists() {
            panic!("Invalid directory")
        }

        let mut episodes = util::read_dir(&path).unwrap();

        if episodes.len() == 0 {
            panic!("Directory contains no media")
        }

        // TODO databae path should be different from show directory
        let mut db = db::Database::init(maybe_path.to_path_buf())?;

        db.insert(&mut episodes, path);
        db.save()?;
        db.print_db();

        Ok(())
    }
}
