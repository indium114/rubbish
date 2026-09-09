use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub id: String,
    pub original_path: String,
	pub stored_name: String,
	pub deleted_at: u64,
	pub is_dir: bool,
}

pub fn rubbish_path() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.rubbish"
}

pub fn load_metadata() -> Vec<Entry> {
    fs::read_to_string(rubbish_path() + "/metadata.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Vec::new())
}

pub fn save_metadata(data: Vec<Entry>) -> bool {
    match serde_json::to_string_pretty(&data) {
        Ok(json) => {
            let path = rubbish_path() + "/metadata.json";
            if let Some(parent) = Path::new(&path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::write(&path, json).is_ok()
        },
        Err(_) => false,
    }
}
