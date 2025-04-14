use std::collections::BTreeMap;

use iota_sdk::{rpc_types::{IotaMoveStruct, IotaMoveValue}, types::base_types::{IotaAddress, ObjectID}};

#[allow(dead_code)]
pub struct Song {
    pub id: ObjectID,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub release_year: usize,
    pub genre: String,
    pub cover_art_url: String,
    pub streaming_price: usize,
    pub owner: IotaAddress,
    pub creator_balance: usize,
    pub distributors: DistributionMap,
    pub display_id: Option<ObjectID>,
}

impl From<IotaMoveStruct> for Song {
    fn from(s: IotaMoveStruct) -> Self {
        Self {
            id: parse_uid(&s, "id"),
            title: parse_string(&s, "title"),
            artist: parse_string(&s, "artist"),
            album: parse_string(&s, "album"),
            release_year: parse_string(&s, "release_year").parse().unwrap(),
            genre: parse_string(&s, "genre"),
            cover_art_url: parse_string(&s, "cover_art_url"),
            streaming_price: parse_string(&s, "streaming_price").parse().unwrap(),
            owner: parse_address(&s, "owner"),
            creator_balance: parse_string(&s, "creator_balance").parse().unwrap(),
            distributors: DistributionMap::from(parse_struct(&s, "distributors")),
            display_id: match s.read_dynamic_field_value("display_id") {
                Some(IotaMoveValue::Address(a)) => Some(ObjectID::from(a)),
                Some(IotaMoveValue::Option(_)) => None,
                _ => panic!("Error parsing display_id from {s}")
            }
        }
    }
}

pub struct SongList(pub Vec<Song>);

impl FromIterator<Song> for SongList {
    fn from_iter<T: IntoIterator<Item = Song>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[allow(dead_code)]
pub struct SongDisplay {
    pub id: ObjectID,
    pub song_id: ObjectID,
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub streaming_price: usize,
    pub cover_art_url: String,
}

impl From<IotaMoveStruct> for SongDisplay {
    fn from(s: IotaMoveStruct) -> Self {
        Self {
            id: parse_uid(&s, "id"),
            song_id: ObjectID::from(parse_address(&s, "song_id")),
            title: parse_string(&s, "title"),
            artist: parse_string(&s, "artist"),
            genre: parse_string(&s, "genre"),
            streaming_price: parse_string(&s, "streaming_price").parse().unwrap(),
            cover_art_url: parse_string(&s, "cover_art_url"),
        }
    }
}

pub struct SongDisplayList(pub Vec<SongDisplay>);

impl FromIterator<SongDisplay> for SongDisplayList {
    fn from_iter<T: IntoIterator<Item = SongDisplay>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

pub struct Distributor {
    pub url: String,
    pub joined_at: usize,
    pub streaming_price: usize,
    pub balance: usize
}

impl From<IotaMoveStruct> for Distributor {
    fn from(s: IotaMoveStruct) -> Self {
        Self {
            url: parse_string(&s, "url"),
            joined_at: parse_string(&s, "joined_at").parse().unwrap(),
            streaming_price: parse_string(&s, "streaming_price").parse().unwrap(),
            balance: parse_string(&s, "balance").parse().unwrap()
        }
    }
}

pub struct DistributionMap(pub BTreeMap<IotaAddress, Distributor>);

impl From<IotaMoveStruct> for DistributionMap {
    fn from(s: IotaMoveStruct) -> Self {
        Self(
            parse_vec(&s, "contents").into_iter()
                .map(|entry| match entry {
                    IotaMoveValue::Struct(s) => (
                        parse_address(&s, "key"),
                        Distributor::from(parse_struct(&s, "value"))
                    ),
                    _ => panic!("Error parsing {entry}")
                }).collect()
        )
    }
}

fn parse_uid(s: &IotaMoveStruct, field_name: &str) -> ObjectID {
    match s.read_dynamic_field_value(field_name) {
        Some(IotaMoveValue::UID { id }) => id,
        _ => panic!("Error parsing {field_name} from {s}")
    }
}

fn parse_string(s: &IotaMoveStruct, field_name: &str) -> String {
    match s.read_dynamic_field_value(field_name) {
        Some(IotaMoveValue::String(parsed)) => parsed,
        _ => panic!("Error parsing {field_name} from {s}")
    }
}

fn parse_address(s: &IotaMoveStruct, field_name: &str) -> IotaAddress {
    match s.read_dynamic_field_value(field_name) {
        Some(IotaMoveValue::Address(parsed)) => parsed,
        _ => panic!("Error parsing {field_name} from {s}")
    }
}

fn parse_struct(s: &IotaMoveStruct, field_name: &str) -> IotaMoveStruct {
    match s.read_dynamic_field_value(field_name) {
        Some(IotaMoveValue::Struct(parsed)) => parsed,
        _ => panic!("Error parsing {field_name} from {s}")
    }
}

fn parse_vec(s: &IotaMoveStruct, field_name: &str) -> Vec<IotaMoveValue> {
    match s.read_dynamic_field_value(field_name) {
        Some(IotaMoveValue::Vector(parsed)) => parsed,
        _ => panic!("Error parsing {field_name} from {s}")
    }
}
