use clap::Parser;
use clap::ValueEnum;
use std::collections::HashMap;
use std::os::unix::prelude::MetadataExt;
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

type Info = HashMap<String, u64>;

fn iterate(path: &str, info: &mut Info) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                iterate(path_str, info)?;
            } else {
                eprintln!("non UTF-8 path: {:?}", path);
            }
        } else {
            let name: String = path.to_str().unwrap().to_string();
            let metadata = fs::metadata(path)?;
            info.insert(name, metadata.size());
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    let mut info = Info::new();
    iterate(&args.folder, &mut info).unwrap();
    println!("{:?}", info);
}
