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
    src: Option<String>,

    #[arg(short, long)]
    dst: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Digest => {
            println!("{}", digest::get(&args.folder.unwrap()));
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
