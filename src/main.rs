mod db;

use std::{
    env,
    error::Error,
    fs::{self},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    // Read the current dir and return all valid videos
    let mut episodes = read_dir().unwrap();

    println!("Command: {:#?}", command);
    println!("Episodes: {:#?}", episodes);

    // NOTE Test stuff, move this to lib eventually
    let mut db = db::Database::init(env::current_dir()?)?;
    db.insert(&mut episodes);
    db.save()?;
    db.series.watch_next()?;
    db.print_db();
    db.save()?;

    Ok(())
}

// TODO Add a comperehensive list of video file formats
const VIDEO_FORMATS: &'static [&'static str] = &[".mkv", ".mp4"];

pub fn is_video_file(file_name: &str) -> bool {
    VIDEO_FORMATS.iter().any(|ext| file_name.ends_with(ext))
}

/// Read a directory and return a list of valid video file names
pub fn read_dir() -> Result<Vec<String>, std::io::Error> {
    let dir = env::current_dir()?;
    let files = fs::read_dir(&dir)?.filter_map(|x| x.ok());

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
