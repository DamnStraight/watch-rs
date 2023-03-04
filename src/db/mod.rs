use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub dir: String,
    episodes: Vec<Episode>,
}

impl Series {
    pub fn new(dir: String) -> Self {
        Series {
            dir,
            episodes: Vec::new(),
        }
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
}

impl Database {
    pub fn init(data_dir: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let data_dir = data_dir.as_ref();
        let path = data_dir.join("watch.db");

        match fs::read(&path) {
            Ok(bytes) => Ok(Database {
                path,
                series: Self::deserialize(&bytes).unwrap(),
            }),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Database {
                path,
                series: Series::new(data_dir.to_str().unwrap().to_string()),
            }),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        let bytes = Self::serialize(&self.series)?;
        let mut file = File::create(&self.path)?;
        file.write_all(&bytes)?;

        Ok(())
    }

    pub fn insert(&mut self, shows: &mut Vec<String>, dir: String) {
        shows.retain(|show| {
            !self
                .series
                .episodes
                .iter()
                .any(|episode| episode.name == *show)
        });

        self.series.episodes = shows
            .iter()
            .map(|name| Episode::new(name.to_string()))
            .collect::<Vec<Episode>>();

        self.series.episodes.sort_by(|a, b| a.name.cmp(&b.name));
        self.series.dir = dir;
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

    /// Launch the first alphabetically ordered episode marked as unwatched
    /// FIXME Clean up this function and change the behavior to launch either
    /// the first or last episode if all are marked as watched
    pub fn watch_next(&mut self) -> Result<(), Box<dyn Error>> {
        let maybe_episode = self
            .series
            .episodes
            .iter_mut()
            .find(|episode| episode.watched == false);

        if let Some(mut episode) = maybe_episode {
            println!("Launching: {}", episode.name);
            // TODO Make the player configurable (currently only works on Mac with IINA installed)
            let execution_result = std::process::Command::new("open")
                .current_dir(&format!("{}/", &self.series.dir))
                .args(&["-a", "IINA", &episode.name])
                .spawn();

            return match execution_result {
                Ok(_) => {
                    episode.watched = true;
                    self.save()?;

                    Ok(())
                }
                Err(e) => Err(Box::new(e)),
            };
        }

        Ok(())
    }

    pub fn watch_up_to(&mut self, episode_number: usize) -> Result<(), String> {
        if self.series.episodes.len() < episode_number {
            return Err("That episode doesn't exist".to_string());
        }

        for i in 0..=(episode_number - 1) {
            self.series.episodes[i].watched = true;
        }

        self.save().unwrap();
        self.print_db();

        return Ok(());
    }
}
