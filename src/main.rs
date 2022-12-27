use clap::Parser;
use clap::ValueEnum;
use std::fs;

mod digest;

#[derive(ValueEnum, Debug, Clone)]
enum Action {
    Digest,
    Diff,
}

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    action: Action,

    #[arg(short, long)]
    folder: Option<String>,

    #[arg(short, long)]
    min_file_size: Option<u64>,

    #[arg(short, long)]
    src: Option<String>,

    #[arg(short, long)]
    dst: Option<String>,
}

/// Known issues:
/// 1. the generated `mkdir -p` could fail due to a path exists
fn main() {
    let args = Args::parse();

    match args.action {
        Action::Digest => {
            let min_file_size = args.min_file_size.unwrap_or(1_000_000);
            println!("{}", digest::get(&args.folder.unwrap(), min_file_size));
        }
        Action::Diff => {
            println!(
                "{}",
                digest::diff(
                    &fs::read_to_string(&args.src.unwrap()).unwrap(),
                    &fs::read_to_string(&args.dst.unwrap()).unwrap()
                )
            );
        }
    }
}
