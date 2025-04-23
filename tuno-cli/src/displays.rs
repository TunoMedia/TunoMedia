use iota_sdk::types::base_types::IotaAddress;
use tabled::{Table, Tabled};
use std::fmt::{Display, Formatter};

use crate::types::*;

#[derive(Tabled)]
pub struct TabledSong {
    id: String,
    title: String,
    artist: String,
    is_available: bool,
    distributors: usize
}

impl From<&Song> for TabledSong {
    fn from(song: &Song) -> Self {
        Self {
            id: song.id.to_string(),
            title: song.title.clone(),
            artist: song.artist.clone(),
            is_available: song.display_id.is_some(),
            distributors: song.distributors.0.len()
        }
    }
}

impl Display for SongList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Table::new(self.0.iter().map(|s| TabledSong::from(s))))
    }
}

#[derive(Tabled)]
pub struct TabledSongDisplay {
    song_id: String,
    title: String,
    artist: String,
    genre: String,
    price: usize
}

impl From<&SongDisplay> for TabledSongDisplay {
    fn from(display: &SongDisplay) -> Self {
        Self {
            song_id: display.song_id.to_string(),
            title: display.title.clone(),
            artist: display.artist.clone(),
            genre: display.genre.clone(),
            price: display.streaming_price
        }
    }
}

impl Display for SongDisplayList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Table::new(self.0.iter().map(|d| TabledSongDisplay::from(d))))
    }
}

#[derive(Tabled)]
struct TabledDistributor {
    address: String,
    url: String,
    joined_at: usize,
    streaming_price: usize,
    balance: usize
}

impl From<(&IotaAddress, &Distributor)> for TabledDistributor {
    fn from((a, d): (&IotaAddress, &Distributor)) -> Self {
        Self {
            address: a.to_string(),
            url: d.url.clone(),
            joined_at: d.joined_at,
            streaming_price: d.streaming_price,
            balance: d.balance
        }
    }
}

impl Display for DistributionMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {        
        write!(f, "{}", Table::new(self.0.iter().map(|entry| TabledDistributor::from(entry))))
    }
}
