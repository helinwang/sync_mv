use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::os::unix::prelude::MetadataExt;
use std::time::UNIX_EPOCH;

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Metadata {
    size: u64,
    modified: u128,
}

#[derive(Deserialize, Serialize)]
struct Summary {
    base_dir: String,
    min_file_bytes: u64,

    // use BTreeMap for deterministic JSON serialization, which helps testing
    files: BTreeMap<String, Metadata>,
}

impl Summary {
    fn new(base_dir: &str, min_file_bytes: u64) -> Self {
        Summary {
            base_dir: base_dir.to_string(),
            min_file_bytes,
            files: BTreeMap::new(),
        }
    }

    fn add_file(&mut self, path: String, size: u64, modified: u128) {
        if size < self.min_file_bytes {
            return;
        }

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

fn iterate(path: &str, summary: &mut Summary) {
    match fs::read_dir(path) {
        Err(err) => eprintln!("can't read dir {} due to {}", path, err),
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Err(err) => eprintln!("can't read entry from dir {} due to {}", path, err),
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            if let Some(path_str) = path.to_str() {
                                iterate(path_str, summary);
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

                            match fs::symlink_metadata(path.clone()) {
                                Err(err) => eprintln!(
                                    "can't get symlink status for {} due to {}",
                                    path.to_str().unwrap(),
                                    err
                                ),
                                Ok(metadata) => {
                                    if metadata.is_symlink() {
                                        return;
                                    }
                                }
                            }

                            match fs::metadata(path.clone()) {
                                Err(err) => {
                                    eprintln!(
                                        "can't read metadata from {} due to {}",
                                        path.to_str().unwrap(),
                                        err
                                    )
                                }
                                Ok(metadata) => summary.add_file(
                                    name,
                                    metadata.size(),
                                    metadata
                                        .modified()
                                        .unwrap()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_nanos(),
                                ),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn get(base_dir: &str, min_file_size: u64) -> String {
    let mut base_dir = base_dir.to_string();
    if !base_dir.ends_with('/') {
        base_dir += "/";
    }

    let mut summary = Summary::new(&base_dir, min_file_size);
    iterate(&base_dir, &mut summary);
    serde_json::to_string_pretty(&summary).unwrap()
}

pub fn diff(src: &str, dst: &str) -> String {
    let src: Summary = serde_json::from_str(src).unwrap();
    let dst: Summary = serde_json::from_str(dst).unwrap();
    let mut metadata_to_path = MetadataToPath::new();
    for (path, metadata) in &dst.files {
        if src.files.contains_key(path) {
            // If the file in dst are still in src, even src copied a file to somewhere else, defer
            // to rsync to do the copy instead of generating a `cp` command to copy locally.
            continue;
        }

        if let Some(existing) = metadata_to_path.insert(*metadata, path.to_string()) {
            eprintln!(
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
    let mut already_moved = HashMap::<String, String>::new();

    for (path, metadata) in &src.files {
        if dst.files.contains_key(path) {
            // Ignore src file alredy in dst, regardless if they are the same or not. This prevents
            // the generated `mv` command overrding anything.
            continue;
        }

        if let Some(dst_path) = metadata_to_path.get(metadata) {
            if let Some(moved_to) = already_moved.get(dst_path) {
                eprintln!(
                    "ignored moving {} to {} since it's already moved to {}",
                    dst_path, path, moved_to
                );
                continue;
            }

            already_moved.insert(dst_path.to_string(), path.to_string());

            moves.push(Move {
                from: format!("{}{}", dst.base_dir, dst_path),
                to: format!("{}{}", dst.base_dir, path),
            });
        }
    }

    let mut lines = Vec::<String>::new();
    lines.push("set -x".to_string()); // print each command when executing
    lines.push("set -e".to_string()); // stop when there is error
    let mut mkdir_done = HashSet::<String>::new();

    for move_file in &moves {
        let parent_dir = &move_file.to[0..move_file.to.rfind('/').unwrap()];
        if mkdir_done.insert(parent_dir.to_string()) {
            lines.push(format!("mkdir -p '{}'", parent_dir));
        }
        lines.push(format!("mv '{}' '{}'", move_file.from, move_file.to));
    }

    lines.join("\n")
}
