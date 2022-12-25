use clap::Parser;
use clap::ValueEnum;
use sync_mv::digest;

#[derive(ValueEnum, Debug, Clone)]
enum Action {
    A,
    B,
}

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    action: Action,

    #[arg(short, long)]
    folder: String,
}

fn main() {
    let args = Args::parse();
    let summary = digest::get(&args.folder).unwrap();
    for (path, size) in summary {
        println!("{} {}", size, path);
    }
}
