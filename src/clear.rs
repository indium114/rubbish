use std::{process, fs};

pub fn clear() {
    match fs::remove_dir_all(crate::models::rubbish_path() + "/files") {
        Ok(_) => {
            let _ = crate::models::save_metadata(Vec::new());
        },
        Err(e) => {
            eprintln!("Failed to clear rubbish: {e}");
            process::exit(1);
        }
    }
}
