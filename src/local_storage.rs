use std::{fs, path::PathBuf};
use anyhow::Result;

pub(crate) fn get_local_song_file(hex_id: &str) -> Result<fs::File> {
    let (p, f) = hex_id.split_at(2);
    let mut location = PathBuf::from("media");

    location.extend([p, f]);
    Ok(fs::File::open(location)?)
}

pub(crate) fn store_song(file: &PathBuf, hex_id: &str) -> Result<PathBuf> {
    let (p, f) = hex_id.split_at(2);
    let mut location = PathBuf::from("media");

    location.extend([p]);
    if !location.is_dir() {
        fs::create_dir(&location)?;
    }

    location.extend([f]);
    fs::copy(&file, &location)?;

    Ok(location)
}
