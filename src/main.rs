use clap::Parser;
use clap::ValueEnum;
use std::{fs, io};

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

fn iterate(path: &str) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            if path.is_dir() {
                iterate(path_str)?;
            } else {
                println!("{}", path_str);
            }
        } else {
            eprintln!("non UTF-8 path: {:?}", path);
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    iterate(&args.folder).unwrap();
}
