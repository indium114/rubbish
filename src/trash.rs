use anyhow::bail;
use indicatif::ProgressBar;
use lz4_flex::frame::FrameEncoder;
use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
    sync::{Arc, LazyLock},
    time::{SystemTime, UNIX_EPOCH},
};
use tar::Builder;
use tokio::sync::Semaphore;

static META_SEMAPHORE: LazyLock<Arc<Semaphore>> = LazyLock::new(|| Arc::new(Semaphore::new(1)));

pub fn dir_size(path: &str) -> u64 {
    let p = Path::new(path);
    if p.is_file() {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }
    fs::read_dir(p)
        .ok()
        .map(|d| {
            d.filter_map(|e| e.ok())
                .map(|e| dir_size(&e.path().to_string_lossy()))
                .sum()
        })
        .unwrap_or(0)
}

fn compress(src: &str, dest: &str, pb: &ProgressBar) -> anyhow::Result<()> {
    pb.set_length(dir_size(src.clone()));
    let output = File::create(dest)?;
    let bufwrite = BufWriter::new(output);
    let tracked = pb.wrap_write(bufwrite);

    let lz4_encoder = FrameEncoder::new(tracked);
    let mut tar_builder = Builder::new(lz4_encoder);

    let path = Path::new(src.clone());
    if path.is_dir() {
        tar_builder.append_dir_all(".", src)?;
    } else if path.is_file() {
        tar_builder
            .append_path_with_name(src, path.file_name().and_then(|o| o.to_str()).unwrap())?;
    } else {
        return Ok(()); // file is a special file
    }

    let mut lz4_encoder = tar_builder.into_inner()?;
    lz4_encoder.finish()?;

    Ok(())
}

pub fn trash(
    file: &str,
    recursive: bool,
    force: bool,
    permanent: bool,
    pb: &ProgressBar,
) -> anyhow::Result<()> {
    match permanent {
        true => match recursive {
            true => fs::remove_dir_all(file)?,
            false => fs::remove_file(file)?,
        },
        false => {
            let path1 = Path::new(file.clone());
            if path1.is_dir() && !recursive {
                bail!("tried to trash a directory without recursive flag");
            }
            let id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("You've done it. You've achieved time-travel")
                .as_nanos();
            let store_name = id.to_string()
                + "_"
                + Path::new(file.clone())
                    .file_name()
                    .and_then(|o| o.to_str())
                    .unwrap();
            let path = crate::models::rubbish_path() + "/files/" + &store_name;

            match compress(file, &path, pb) {
                Ok(_) => {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let permit = rt.block_on(async { META_SEMAPHORE.acquire().await.unwrap() });
                    let mut meta = crate::models::load_metadata();
                    meta.push(crate::models::Entry {
                        id: id.to_string(),
                        original_path: file.to_string(),
                        stored_name: path,
                        deleted_at: id,
                        is_dir: path1.is_dir(),
                    });
                    let _ = crate::models::save_metadata(meta);
                    drop(permit);
                    match path1.is_dir() {
                        true => fs::remove_dir_all(path1)?,
                        false => fs::remove_file(path1)?,
                    }
                }
                Err(e) => match force {
                    true => (),
                    false => return Err(e),
                },
            };
        }
    }

    Ok(())
}
