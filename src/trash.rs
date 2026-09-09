use std::{
    fs::{self, File},
    path::Path,
    io::BufWriter,
    time::{
        SystemTime,
        UNIX_EPOCH
    },
};
use tar::Builder;
use lz4_flex::frame::FrameEncoder;

fn rubbish_path() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.rubbish"
}

fn compress(src: &str, dest: &str) -> anyhow::Result<()> {
    let output = File::create(dest)?;
    let bufwrite = BufWriter::new(output);

    let lz4_encoder = FrameEncoder::new(bufwrite);
    let mut tar_builder = Builder::new(lz4_encoder);

    let path = Path::new(src.clone());
    if path.is_dir() {
        tar_builder.append_dir_all(".", src)?;
    } else if path.is_file() {
        tar_builder.append_path_with_name(src, path.file_name().and_then(|o| o.to_str()).unwrap())?;
    } else {
        return Ok(()); // file is a special file
    }

    let mut lz4_encoder = tar_builder.into_inner()?;
    lz4_encoder.finish()?;

    Ok(())
}

pub fn trash(file: &str, recursive: bool, force: bool, permanent: bool) -> anyhow::Result<()> {
    match permanent {
        true => match recursive {
            true => fs::remove_dir_all(file)?,
            false => fs::remove_file(file)?,
        },
        false => {
            let id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("You've done it. You've achieved time-travel")
                .as_nanos();
            let store_name = id.to_string() + "_" + Path::new(file.clone()).file_name().and_then(|o| o.to_str()).unwrap();
            let path = rubbish_path() + "/files/" + &store_name;

            match compress(file, &path) {
                Ok(_) => (),
                Err(e) => match force {
                    true => (),
                    false => return Err(e),
                }
            };
            println!("Compressed the {} to {}", file.clone(), path.clone())
        }
    }

    Ok(())
}
