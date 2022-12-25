use std::collections::HashMap;
use std::os::unix::prelude::MetadataExt;
use std::{fs, io};

pub type Summary = HashMap<String, u64>;

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
            summary.insert(name, metadata.size());
        }
    }

    Ok(())
}

pub fn get(path: &str) -> Result<Summary, io::Error> {
    let mut summary = Summary::new();
    iterate(path, &mut summary)?;
    Ok(summary)
}
