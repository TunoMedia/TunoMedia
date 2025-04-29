use std::str::FromStr as _;

use iota_sdk::types::base_types::ObjectID;
use tuno_cli::client::{Client, Connection};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
    .invoke_handler(tauri::generate_handler![list_distributors])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// TODO: client as managed state
// https://v2.tauri.app/develop/calling-rust/#accessing-managed-state

#[tauri::command]
async fn list_distributors() -> Result<(), String> {
  let Ok(package_id) = ObjectID::from_str("0x31554108d503d15bb2e4c99abeb8001053feb3e44257d226d940a8fe02312173") else {
    return Err("Couldn't create connection".to_string());
  };

  let Ok(client) = Client::new(Connection { config: None, package_id }) else {
    return Err("Couldn't create client".to_string());
  };

  let Ok(song_id) = ObjectID::from_str("0x7463fa6bbbb2217298255e0dace21e2d00a8116b767fc55d371a08c51dd28573") else {
    return Err("Couldn't create song_id".to_string());
  };

  let Ok(song) = client.get_song(song_id).await else {
    return Err("Couldn't get song".to_string());
  };
  
  println!("DISTRIBUTORS: {}", song.distributors);
  Ok(())
}
