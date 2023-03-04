use std::fs;

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
