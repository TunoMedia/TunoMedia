use std::str::FromStr as _;

use iota_sdk::types::base_types::ObjectID;
use tuno_cli::client::{Client, Connection};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let Ok(package_id) = ObjectID::from_str("0x31554108d503d15bb2e4c99abeb8001053feb3e44257d226d940a8fe02312173") else {
    panic!("Couldn't create package_id");
  };

  let Ok(client) = Client::new(Connection { config: None, package_id }) else {
    panic!("Couldn't create client");
  };

  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .manage(client)
    .invoke_handler(tauri::generate_handler![get_distributor])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn get_distributor(
  song_id: &str,
  state: tauri::State<'_, Client>
) -> Result<(String, String), String> {
  println!("Listing {}...", song_id);

  let Ok(song_obj) = ObjectID::from_str(song_id) else {
    return Err("Couldn't create song_obj".to_string());
  };

  let Ok(song) = state.get_song(song_obj).await else {
    return Err("Couldn't get song".to_string());
  };
  
  println!("DISTRIBUTORS: {}", song.distributors);
  let Some((
    addr,
    distributor
  )) = song.distributors.get_first() else {
    return Err("Couldn't find a distributor".to_string());
  };

  let Ok(
    tx
  ) = state.get_payment_transaction(song_obj, addr).await else {
    return Err("Couldn't create payment transaction".to_string());
  };

  let Ok(raw_transaction) = bcs::to_bytes(&tx) else {
    return Err("Couldn't serialize payment transaction".to_string());
  };

  Ok((distributor.url.clone(), hex::encode(raw_transaction)))
}
