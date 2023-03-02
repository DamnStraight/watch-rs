mod cmd;
mod db;

use clap::Parser;
use std::{
    env,
    error::Error,
    fs::{self},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = cmd::Cli::parse();

    match args.command {
        cmd::Commands::Add { path } => {
            let path = path
                .as_deref()
                .map(|s| s.to_str().unwrap().to_string())
                .unwrap_or(env::current_dir().unwrap().to_str().unwrap().to_string());

            let maybe_path = Path::new(&path);

            if !maybe_path.exists() {
                panic!("Invalid directory")
            }

            let mut episodes = read_dir(&path).unwrap();

            if episodes.len() == 0 {
                panic!("Directory contains no media")
            }

            // TODO databae path should be different from show directory
            let mut db = db::Database::init(maybe_path.to_path_buf())?;

            db.insert(&mut episodes, path);
            db.save()?;
            db.print_db();
        }
        cmd::Commands::Next => {
            let mut db = db::Database::init(env::current_dir()?)?;
            db.watch_next()?;
        },
        cmd::Commands::Upto { episode } => {
            let mut db = db::Database::init(env::current_dir()?)?;
            db.watch_up_to(episode)?;
        }
    }

    Ok(())
}

// TODO Add a comperehensive list of video file formats
const VIDEO_FORMATS: &'static [&'static str] = &[".mkv", ".mp4"];

pub fn is_video_file(file_name: &str) -> bool {
    VIDEO_FORMATS.iter().any(|ext| file_name.ends_with(ext))
}

/// Read a directory and return a list of valid video file names
pub fn read_dir(path: &str) -> Result<Vec<String>, std::io::Error> {
    let files = fs::read_dir(&path)?.filter_map(|x| x.ok());

    let mut episodes: Vec<String> = Vec::new();

    for file in files {
        if let Some(file_name) = file.file_name().to_str() {
            if is_video_file(file_name) {
                episodes.push(String::from(file_name));
            }
        }
    }

    Ok(episodes)
}
