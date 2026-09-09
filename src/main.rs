use clap::Parser;
use std::process;

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
        for file in &cli.files {
            crate::trash::trash(
                &file,
                cli.recursive,
                cli.force,
                cli.permanent
            )?
        }
    };

    Ok(())
}
