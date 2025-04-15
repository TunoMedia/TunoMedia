use std::io::{BufReader, Read};
use std::{fs, path::PathBuf};
use anyhow::{bail, Result};

use sha2::{Sha256, Digest};
use symphonia::default::get_probe;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;

const DEFAULT_MEDIA_STORAGE: &str = "media";

#[derive(Debug)]
pub struct FileMetadata {
    length: usize,
    duration: usize,
    signature: Vec<Vec<u8>>,
}

impl From<&PathBuf> for FileMetadata {
    fn from(path: &PathBuf) -> Self {
        Self {
            length: fs::metadata(path).expect("Error parsing metadata").len() as usize,
            duration: compute_duration(path).expect("Error computing duration") as usize,
            signature: compute_signature(path).expect("Error computing duration")
        }
    }
}

impl FileMetadata {
    pub(crate) fn as_arguments(
        &self,
        ptb: &mut ProgrammableTransactionBuilder
    ) -> Result<Vec<Argument>> {
        Ok(vec![
            ptb.pure(&self.length)?,
            ptb.pure(&self.duration)?,
            ptb.pure(&self.signature)?,
        ])
    }
}

fn compute_duration(path: &PathBuf) -> Result<u64> {
    let file = fs::File::open(path).expect("failed to open media");
    let probe_result = get_probe().format(
        &Hint::new().with_extension("mp3"),
        MediaSourceStream::new(
            Box::new(file),
            Default::default()
        ),
        &Default::default(),
        &Default::default()
    ).expect("Error probing file");

    let track = probe_result.format
        .default_track().expect("no default track");

    if let Some(n_frames) = track.codec_params.n_frames {
        if let Some(sample_rate) = track.codec_params.sample_rate {
            return Ok((n_frames * 1000) / sample_rate as u64);
        }
    }

    bail!("Error extracting codec parameters")
}

fn compute_signature(path: &PathBuf) -> Result<Vec<Vec<u8>>> {
    let file = fs::File::open(path).expect("failed to open media");
    let mut reader = BufReader::new(file);
    let mut sig = vec![];

    let mut buf = vec![0; 512*512];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 { break }

        let mut hasher = Sha256::new();
        hasher.update(buf[..n].to_vec());

        sig.push(hasher.finalize().to_vec());
    }

    Ok(sig)
}

pub(crate) fn get_local_song_reader(hex_id: &str) -> Result<BufReader<fs::File>> {
    let (p, f) = hex_id.split_at(2);
    let mut location = PathBuf::from(DEFAULT_MEDIA_STORAGE);

    location.extend([p, f]);
    Ok(BufReader::new(fs::File::open(location)?))
}

pub(crate) fn get_all_song_ids() -> Result<Vec<String>> {
    let location = PathBuf::from(DEFAULT_MEDIA_STORAGE);

    Ok(
        fs::read_dir(location)?
            .flat_map(|prefix| fs::read_dir(prefix.unwrap().path()).unwrap())
            .map(|path| 
                "0x".to_string() + &path.unwrap().path().to_str().unwrap()
                .split("/")
                .collect::<Vec<_>>()[1..]
                .join("")
            ).collect()
    )
}

pub(crate) fn store_song_from_file(file: &PathBuf, hex_id: &str) -> Result<PathBuf> {
    let (p, f) = hex_id.split_at(2);
    let location = get_and_create_media_file(vec![p, f])?;

    fs::copy(&file, &location)?;

    Ok(location)
}

pub(crate) fn store_song_from_bytes(data: Vec<u8>, hex_id: &str) -> Result<PathBuf> {
    let (p, f) = hex_id.split_at(2);
    let location = get_and_create_media_file(vec![p, f])?;

    fs::write(&location, data)?;

    Ok(location)
}

fn get_and_create_media_file(path: Vec<&str>) -> Result<PathBuf> {
    let mut location = PathBuf::from(DEFAULT_MEDIA_STORAGE);
    for p in path {
        if !location.is_dir() {
            fs::create_dir(&location)?;
        }

        location.extend([p]);
    }
    
    Ok(location)
}
