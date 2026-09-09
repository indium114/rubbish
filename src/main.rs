use clap::Parser;
use indicatif::{MultiProgress, ProgressStyle, ProgressBar, ProgressState};
use rayon::prelude::*;
use std::{fs, process, path::Path, io::Write, time::Duration};

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
    let cli = Cli::parse();

    let mp = MultiProgress::new();

    let _ = if cli.list {
        todo!()
    } else if let Some(id) = &cli.restore {
        todo!()
    } else if let Some(id) = &cli.delete {
        todo!()
    } else if cli.clear {
        todo!()
    } else if cli.files.is_empty() {
        eprintln!("No files provided");
        process::exit(1);
    } else {
        cli.files.par_iter().for_each(|file| {
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
        });
    };

    Ok(())
}
