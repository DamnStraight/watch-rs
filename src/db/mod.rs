use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    dir: String,
    episodes: Vec<Episode>,
}

impl Series {
    pub fn new(dir: String) -> Self {
        Series {
            dir,
            episodes: Vec::new(),
        }
    }

    /// Launch the first alphabetically ordered episode marked as unwatched
    /// FIXME Clean up this function and change the behavior to launch either
    /// the first or last episode if all are marked as watched
    pub fn watch_next(&mut self) -> Result<(), Box<dyn Error>> {
        let maybe_episode = self
            .episodes
            .iter_mut()
            .find(|episode| episode.watched == false);

        if let Some(mut episode) = maybe_episode {
            println!("Launching: {}", episode.name);
            // TODO Make the player configurable (currently only works on Mac with IINA installed)
            let execution_result = std::process::Command::new("open")
                .current_dir(&format!("{}/", &self.dir))
                .args(&["-a", "IINA", &episode.name])
                .spawn();

            return match execution_result {
                Ok(_) => {
                    episode.watched = true;
                    Ok(())
                }
                Err(e) => Err(Box::new(e)),
            };
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Episode {
    name: String,
    watched: bool,
}

impl Episode {
    pub fn new(name: String) -> Self {
        Episode {
            name,
            watched: false,
        }
    }
}

pub struct Database {
    path: PathBuf,
    pub series: Series,
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
                series: Self::deserialize(&bytes).unwrap(),
                bytes,
                dirty: false,
            }),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Database {
                path,
                bytes: Vec::new(),
                series: Series::new(data_dir.to_str().unwrap().to_string()),
                dirty: false,
            }),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.dirty {
            return Ok(());
        }

        let bytes = Self::serialize(&self.series)?;
        let mut file = File::create("watch.db")?;
        file.write_all(&bytes)?;
        self.dirty = false;

        Ok(())
    }

    pub fn insert(&mut self, shows: &mut Vec<String>) {
        shows.retain(|show| {
            !self
                .series
                .episodes
                .iter()
                .any(|episode| episode.name == *show)
        });

        self.series.episodes.append(
            &mut shows
                .iter()
                .map(|name| Episode::new(name.to_string()))
                .collect::<Vec<Episode>>(),
        );

        self.series.episodes.sort_by(|a, b| a.name.cmp(&b.name));

        self.dirty = true;
    }

    pub fn serialize(data: &Series) -> Result<Vec<u8>, Box<dyn Error>> {
        let encoded: Vec<u8> = bincode::serialize(&data)?;

        Ok(encoded)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Series, Box<dyn Error>> {
        let decoded: Series = bincode::deserialize(bytes)?;

        Ok(decoded)
    }

    pub fn print_db(&self) {
        println!("DB: {:#?}", self.series);
    }
}
