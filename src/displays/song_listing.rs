use iota_sdk::rpc_types::IotaMoveStruct;
use tabled::{Table, Tabled};
use std::fmt::{Display, Formatter};

#[derive(Tabled)]
pub struct SongListing {
    id: String,
    title: String,
    artist: String,
    is_available: bool,
    distributors: u64
}

impl From<IotaMoveStruct> for SongListing {
    fn from(s: IotaMoveStruct) -> Self {
        Self {
            id: s.read_dynamic_field_value("id").unwrap().to_string(),
            title: s.read_dynamic_field_value("title").unwrap().to_string(),
            artist: s.read_dynamic_field_value("artist").unwrap().to_string(),
            // TODO: Test unavailability
            is_available: s.read_dynamic_field_value("display_id").is_some(),
            // TODO: Implement (Might need Display function on smart contract)
            distributors: 0
        }
    }
}

pub struct SongList(Vec<SongListing>);

impl SongList {
    pub fn from(list: Vec<SongListing>) -> Self {
        Self(list)
    }
}

impl Display for SongList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Table::new(&self.0))
    }
}
