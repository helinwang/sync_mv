use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::os::unix::prelude::MetadataExt;
use std::time::UNIX_EPOCH;
use std::{fs, io};

pub type Summary = HashMap<String, Metadata>;

type MetadataToPath = HashMap<Metadata, String>;

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Metadata {
    pub size: u64,
    pub modified: u128,
}

pub struct Move {
    pub from: String,
    pub to: String,
}

// TODO: remove the starting directory prefix
fn iterate(path: &str, summary: &mut Summary) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                iterate(path_str, summary)?;
            } else {
                eprintln!("non UTF-8 path: {:?}", path);
            }
        } else {
            let name: String = path.to_str().unwrap().to_string();
            let metadata = fs::metadata(path)?;

            summary.insert(
                name,
                Metadata {
                    size: metadata.size(),
                    modified: metadata
                        .modified()?
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos(),
                },
            );
        }
    }

    Ok(())
}

pub fn get(path: &str) -> Result<Summary, io::Error> {
    let mut summary = Summary::new();
    iterate(path, &mut summary)?;
    Ok(summary)
}

pub fn diff(src: &Summary, dst: &Summary) -> Vec<Move> {
    let mut metadata_to_path = MetadataToPath::new();
    for (path, metadata) in dst {
        if src.contains_key(path) {
            // If the file in dst are still in src, even src copied a file to somewhere else, defer
            // to rsync to do the copy instead of generating a `cp` command to copy locally.
            continue;
        }

        if let Some(existing) = metadata_to_path.insert(*metadata, path.to_string()) {
            println!(
                "replacing existing path from dst {} with {}",
                existing, path
            );
        }
    }

    for (path, metadata) in src {
        if let Some(dst_path) = metadata_to_path.get(metadata) {
            println!("mv {} {}", dst_path, path);
        }
    }

    vec![]
}
