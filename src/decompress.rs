use crate::{trash::dir_size, models::Entry};
use indicatif::{ProgressBar, ProgressStyle};
use lz4_flex::frame::FrameDecoder;
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
    time::Duration,
};
use tar::Archive;

fn decompress(src: &str, dest: &str, pb: &ProgressBar, dir: bool) -> anyhow::Result<()> {
    pb.set_length(dir_size(src.clone()));
    pb.set_message(dest.to_string());
    let input = File::open(src)?;
    let bufread = BufReader::new(input);
    let tracked = pb.wrap_read(bufread);

    let lz4_decoder = FrameDecoder::new(tracked);
    let mut tar_archive = Archive::new(lz4_decoder);

    match dir {
        true => {
            fs::create_dir_all(dest)?;
            tar_archive.unpack(dest)?;
        },
        false => {
            let mut entries = tar_archive.entries()?;
            let mut entry = entries
                .next()
                .ok_or_else(|| anyhow::anyhow!("archive is empty??"))??;

            if let Some(parent) = Path::new(dest).parent() {
                fs::create_dir_all(parent)?;
            }
            entry.unpack(dest)?;
        }
    }

    fs::remove_file(src)?;

    Ok(())
}

pub fn restore(id: String) -> anyhow::Result<()> {
    let mut entries = crate::models::load_metadata();
    let found_pos = entries.iter().position(|e| e.id == id).expect("No such ID");
    let found = entries.remove(found_pos);
    crate::models::save_metadata(entries);

    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.white}] ({eta})",
    )
    .unwrap()
    .progress_chars("██░"));
    pb.enable_steady_tick(Duration::from_millis(120));

    decompress(&found.stored_name, &found.original_path, &pb, found.is_dir)?;
    Ok(())
}
