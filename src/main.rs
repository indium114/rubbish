use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{
    fs,
    process,
    time::Duration,
};
use tabled::Table;

mod clear;
mod decompress;
mod models;
mod trash;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "rubbish")]
#[command(version = &VERSION)]
#[command(about = "A CLI file trash tool", long_about = None)]
struct Cli {
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    #[arg(short = 'f', long = "force")]
    force: bool,

    #[arg(short = 'p', long = "permanent")]
    permanent: bool,

    #[arg(long = "list")]
    list: bool,

    #[arg(long = "restore", value_name = "id")]
    restore: Option<String>,

    #[arg(long = "delete", value_name = "id")]
    delete: Option<String>,

    #[arg(long = "clear")]
    clear: bool,

    #[arg(trailing_var_arg = true)]
    files: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    fs::create_dir_all(crate::models::rubbish_path() + "/files")?;

    let cli = Cli::parse();

    let mp = MultiProgress::new();

    if cli.list {
        let meta = crate::models::load_metadata();
        println!("{}", Table::new(meta));
    } else if let Some(id) = &cli.restore {
        crate::decompress::restore(id.to_string())?;
    } else if let Some(id) = &cli.delete {
        crate::clear::delete(id.to_string())?;
    } else if cli.clear {
        crate::clear::clear();
    } else if cli.files.is_empty() {
        eprintln!("No files provided");
        process::exit(1);
    } else {
        cli.files
            .par_iter()
            .for_each(|file| match fs::canonicalize(file) {
                Ok(file) => {
                    let file = file.to_str().map(|s| s.to_string()).unwrap();
                    let pb = mp.add(ProgressBar::new(0));
                    pb.set_style(ProgressStyle::with_template(
                        "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.white}] ({eta})",
                    )
                    .unwrap()
                    .progress_chars("██░"));
                    pb.set_message(file.clone());
                    pb.enable_steady_tick(Duration::from_millis(120));

                    match crate::trash::trash(&file, cli.recursive, cli.force, cli.permanent, &pb) {
                        Ok(_) => pb.finish_with_message(format!("{} <done>", file.clone())),
                        Err(e) => pb.finish_with_message(e.to_string()),
                    };
                }
                Err(_) => {
                    eprintln!("File {file} does not exist or is inaccessible");
                    process::exit(1);
                }
            });
    };

    Ok(())
}
