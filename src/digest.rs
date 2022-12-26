use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::os::unix::prelude::MetadataExt;
use std::time::UNIX_EPOCH;
use std::{fs, io};

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Metadata {
    pub size: u64,
    pub modified: u128,
}

#[derive(Deserialize, Serialize)]
pub struct Summary {
    base_dir: String,
    pub files: HashMap<String, Metadata>,
}

impl Summary {
    pub fn new(base_dir: String) -> Self {
        Summary {
            base_dir,
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, path: String, size: u64, modified: u128) {
        let relative_path = path[path
            .find(&self.base_dir)
            .expect("path must contain base dir")
            + self.base_dir.len()..]
            .to_string();
        self.files
            .insert(relative_path, Metadata { size, modified });
    }
}

type MetadataToPath = HashMap<Metadata, String>;

fn iterate(path: &str, summary: &mut Summary) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                iterate(path_str, summary)?;
            } else {
                eprintln!("ignored non UTF-8 folder: {:?}", path);
            }
        } else {
            let name: String = if let Some(path_str) = path.to_str() {
                path_str.to_string()
            } else {
                eprintln!("ignored non UTF-8 file: {:?}", path);
                continue;
            };

            let metadata = fs::metadata(path)?;

            summary.add_file(
                name,
                metadata.size(),
                metadata
                    .modified()?
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
        }
    }

    Ok(())
}

pub fn get(path: &str) -> Result<Summary, io::Error> {
    let mut summary = Summary::new(path.to_string());
    iterate(path, &mut summary)?;
    Ok(summary)
}

pub fn diff(src: &Summary, dst: &Summary) -> String {
    let mut metadata_to_path = MetadataToPath::new();
    for (path, metadata) in &dst.files {
        if src.files.contains_key(path) {
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

    struct Move {
        from: String,
        to: String,
    }
    let mut moves = Vec::<Move>::new();

    for (path, metadata) in &src.files {
        if let Some(dst_path) = metadata_to_path.get(metadata) {
            moves.push(Move {
                from: format!("{}{}", dst.base_dir, dst_path),
                to: format!("{}{}", dst.base_dir, path),
            });
        }
    }

    let mut lines = Vec::<String>::new();
    let mut mkdir_done = HashSet::<String>::new();

    for move_file in &moves {
        let parent_dir = &move_file.to[0..move_file.to.rfind("/").unwrap()];
        if mkdir_done.insert(parent_dir.to_string()) {
            lines.push(format!("mkdir -p '{}'", parent_dir));
        }
        lines.push(format!("mv '{}' '{}'", move_file.from, move_file.to));
    }

    lines.join("\n")
}
