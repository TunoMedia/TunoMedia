use tokio_tungstenite::tungstenite::Message;
use std::fs;

pub fn error_response(message: &str) -> Message {
    Message::Text(
        serde_json::json!({
            "status": "error",
            "message": message
        }).to_string().into()
    )
}

pub fn success_response(message: &str) -> Message {
    Message::Text(
        serde_json::json!({
            "status": "success",
            "message": message
        }).to_string().into()
    )
}

// TODO: get object_id as bytes
pub fn get_file(object_id: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(format!("./media/{object_id}"))
}
