use clap::Parser;
use clap::ValueEnum;
use serde_json;
use std::fs;
use sync_mv::digest;

#[derive(ValueEnum, Debug, Clone)]
enum Action {
    Digest,
    Generate,
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

fn load_summary(path: &str) -> digest::Summary {
    let content = fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Digest => {
            let summary = digest::get(&args.folder.unwrap()).unwrap();
            print!("{}", serde_json::to_string(&summary).unwrap());
        }
        Action::Generate => {
            let src = load_summary(&args.src.unwrap());
            let dst = load_summary(&args.dst.unwrap());
            digest::diff(&src, &dst);
        }
    }
}
