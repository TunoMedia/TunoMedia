use iota_sdk::rpc_types::{IotaMoveStruct, IotaMoveValue};
use tabled::{Table, Tabled};
use std::fmt::{Display, Formatter};

#[derive(Tabled)]
pub struct SongListing {
    id: String,
    title: String,
    artist: String,
    is_available: bool,
    distributors: usize
}

impl From<IotaMoveStruct> for SongListing {
    fn from(s: IotaMoveStruct) -> Self {
        Self {
            id: match s.read_dynamic_field_value("id") {
                Some(IotaMoveValue::UID { id }) => id.to_string(),
                _ => unreachable!("Error parsing id")
            },
            title: match s.read_dynamic_field_value("title") {
                Some(IotaMoveValue::String(title)) => title,
                _ => unreachable!("Error parsing title")
            },
            artist: match s.read_dynamic_field_value("artist") {
                Some(IotaMoveValue::String(artist)) => artist,
                _ => unreachable!("Error parsing artist")
            },
            is_available: match s.read_dynamic_field_value("display_id") {
                Some(IotaMoveValue::Address(_)) => true,
                Some(IotaMoveValue::Option(_)) => false,
                _ => unreachable!("Error parsing display_id")
            },
            distributors: match s.read_dynamic_field_value("distributors") {
                Some(IotaMoveValue::Struct(obj)) => match obj.read_dynamic_field_value("contents") {
                    Some(IotaMoveValue::Vector(distributors)) => distributors.len(),
                    _ => unreachable!("Error parsing distributors value")
                }
                _ => unreachable!("Error parsing distributors vector")
            }
        }
    }
}

#[derive(Tabled)]
pub struct DisplayListing {
    song_id: String,
    title: String,
    artist: String,
    genre: String,
    price: usize
}

impl From<IotaMoveStruct> for DisplayListing {
    fn from(s: IotaMoveStruct) -> Self {
        Self {
            song_id: match s.read_dynamic_field_value("song_id") {
                Some(IotaMoveValue::Address(id)) => id.to_string(),
                _ => unreachable!("Error parsing id")
            },
            title: match s.read_dynamic_field_value("title") {
                Some(IotaMoveValue::String(title)) => title,
                _ => unreachable!("Error parsing title")
            },
            artist: match s.read_dynamic_field_value("artist") {
                Some(IotaMoveValue::String(artist)) => artist,
                _ => unreachable!("Error parsing artist")
            },
            genre: match s.read_dynamic_field_value("genre") {
                Some(IotaMoveValue::String(genre)) => genre,
                _ => unreachable!("Error parsing genre")
            },
            price: match s.read_dynamic_field_value("streaming_price") {
                Some(IotaMoveValue::String(price)) => price.parse().unwrap(),
                _ => unreachable!("Error parsing genre")
            },
        }
    }
}

pub enum SongList {
    Songs(Vec<SongListing>),
    Displays(Vec<DisplayListing>)
}

impl From<Vec<SongListing>> for SongList {
    fn from(list: Vec<SongListing>) -> Self {
        Self::Songs(list)
    }
}

impl From<Vec<DisplayListing>> for SongList {
    fn from(list: Vec<DisplayListing>) -> Self {
        Self::Displays(list)
    }
}

impl Display for SongList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Songs(list) => write!(f, "{}", Table::new(list)),
            Self::Displays(list) => write!(f, "{}", Table::new(list))
        }
    }
}
