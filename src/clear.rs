use std::{fs, process};

pub fn clear() {
    match fs::remove_dir_all(crate::models::rubbish_path() + "/files") {
        Ok(_) => {
            let _ = crate::models::save_metadata(Vec::new());
        }
        Err(e) => {
            eprintln!("Failed to clear rubbish: {e}");
            process::exit(1);
        }
    }
}

pub fn delete(id: String) -> anyhow::Result<()> {
    let mut entries = crate::models::load_metadata();
    let found_pos = entries.iter().position(|e| e.id == id).expect("No such ID");
    let found = entries.remove(found_pos);
    crate::models::save_metadata(entries);

    fs::remove_file(found.stored_name)?;

    Ok(())
}
