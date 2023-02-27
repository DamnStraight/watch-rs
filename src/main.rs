use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[0];

    let mut episodes = read_dir().unwrap();

    println!("Command: {:#?}", command);
    println!("Episodes: {:#?}", episodes);

    let mut db = Database::init(env::current_dir()?)?;
    db.insert_many(&mut episodes);
    db.save()?;
    
    db.print_db();

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

struct Database {
    path: PathBuf,
    shows: Vec<String>,
    bytes: Vec<u8>,
    dirty: bool,
}

impl Database {
    pub fn init(data_dir: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let data_dir = data_dir.as_ref();
        let path = data_dir.join("watch.db");

        match fs::read(&path) {
            Ok(bytes) => Ok(Database {
                path,
                shows: Self::deserialize(&bytes).unwrap(),
                bytes,
                dirty: false,
            }),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Database {
                path,
                bytes: Vec::new(),
                shows: Vec::new(),
                dirty: false,
            }),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.dirty {
            return Ok(());
        }

        let bytes = Self::serialize(&self.shows)?;
        let mut file = File::create("watch.db")?;
        file.write_all(&bytes)?;

        Ok(())
    }

    pub fn insert_many(&mut self, shows: &mut Vec<String>) {
        shows.retain(|show| !self.shows.contains(show));

        self.shows.append(shows);
        self.dirty = true;
    }

    pub fn serialize(data: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {
        let encoded: Vec<u8> = bincode::serialize(&data)?;

        Ok(encoded)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let decoded: Vec<String> = bincode::deserialize(bytes)?;

        Ok(decoded)
    }

    pub fn print_db(&self) {
        for show in &self.shows {
            println!("->{}", show);
        }
    }
}
