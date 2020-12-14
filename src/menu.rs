extern crate parking_lot;

use parking_lot::Mutex;
use std::cmp::Ordering;
use std::fs::Metadata;
use std::result::Result;
use std::sync::Arc;
use std::vec::Vec;
use std::{fs, io, time};

// Default directory to search if src_path: None is provided to expand_path()
const DEFAULT_DIR: &'static str = "./";

// Expand path into list of files consumable by the menu
pub fn expand_path(
    src_path: Option<String>,
    exts: &Vec<&str>,
    output: Arc<Mutex<String>>,
) -> Result<(), io::Error> {
    let path = match src_path {
        Some(src_path) => src_path,
        None => String::from(DEFAULT_DIR),
    };

    let mut file_vec: Vec<(u64, String)> = Vec::new();

    let canon_path = fs::canonicalize(path)?;
    if exts.contains(&super::INCLUDE_DIRS) {
        let meta = canon_path.metadata()?;
        let ts = get_timestamp(meta);
        let dir_file = format!("{}/", canon_path.to_str().unwrap());
        let mut new_file: Vec<(u64, String)> = vec![(ts, String::from(dir_file))];
        file_vec.append(&mut new_file);
    }
    let mut count = 0;
    if let Ok(paths) = fs::read_dir(canon_path) {
        for cur_path in paths {
            if let Ok(cur_path) = cur_path {
                if let Ok(file_type) = cur_path.file_type() {
                    let path = cur_path.path();
                    let cur_file = path.to_str().unwrap();
                    let meta = cur_path.metadata()?;
                    let ts = get_timestamp(meta);

                    if file_type.is_file() && ends_with(cur_file, &exts) {
                        let mut new_file: Vec<(u64, String)> = vec![(ts, String::from(cur_file))];
                        file_vec.append(&mut new_file);
                        count += 1;
                    // file_map.insert(ts, String::from(cur_file));
                    } else if file_type.is_dir() {
                        expand_path(Some(String::from(cur_file)), exts, output.clone())?;
                    }
                }
            }
        }
    }

    if exts.len() != 1 && exts.contains(&super::INCLUDE_DIRS) && count == 0 {
        file_vec.drain(0..1);
    }
    // Sort by timestamp and append to output
    //    file_vec.sort_by_key(|ref k| k.0);
    file_vec.sort_unstable_by(|x, y| {
        if x.1.ends_with("/") {
            return Ordering::Less;
        }
        return x.0.cmp(&y.0);
    });
    for file in file_vec.iter() {
        eprintln!("{:?}", file);
        output.clone().lock().push_str(file.1.as_str());
        output.clone().lock().push_str("\n");
    }
    eprintln!("\n\n");

    Ok(())
}

// Check the filename contains the correct extensios
pub fn ends_with(file_name: &str, extensions: &Vec<&str>) -> bool {
    for ext in extensions {
        if file_name.ends_with(ext) {
            return true;
        }
    }
    false
}

pub fn get_timestamp(meta: Metadata) -> u64 {
    let file_time = meta.modified().unwrap();

    match file_time.duration_since(time::SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
