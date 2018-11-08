use std::vec::Vec;
use std::{fs, io, time};

// Default directory to search if src_path: None is provided to expand_path()
const DEFAULT_DIR: &'static str = "./";

// Expand path into list of files consumable by the menu
pub fn expand_path(
    src_path: Option<String>,
    exts: &Vec<&str>,
    output: &mut String,
) -> Result<(), io::Error> {
    let path = match src_path {
        Some(src_path) => src_path,
        None => String::from(DEFAULT_DIR),
    };

    let mut file_vec: Vec<(u64, String)> = Vec::new();

    let canon_path = fs::canonicalize(path)?;
    if let Ok(paths) = fs::read_dir(canon_path) {
        for cur_path in paths {
            if let Ok(cur_path) = cur_path {
                if let Ok(file_type) = cur_path.file_type() {
                    let path = cur_path.path();
                    let cur_file = path.to_str().unwrap();
                    let mut ts = get_timestamp(&cur_path);
                    if file_type.is_file() && ends_with(cur_file, &exts) {
                        let mut new_file: Vec<(u64, String)> = vec![(ts, String::from(cur_file))];
                        file_vec.append(&mut new_file);
                    // file_map.insert(ts, String::from(cur_file));
                    } else if file_type.is_dir() {
                        expand_path(Some(String::from(cur_file)), exts, output)?;
                    }
                }
            }
        }
    }

    // Sort by timestamp and append to output
    //    file_vec.sort_by_key(|ref k| k.0);
    file_vec.sort_by(|x, y| x.0.cmp(&y.0));
    for file in file_vec {
        output.push_str(file.1.as_str());
        output.push_str("\n");
    }

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

pub fn get_timestamp(file_path: &fs::DirEntry) -> u64 {
    let meta = file_path.metadata();

    let file_time = meta.unwrap().modified().unwrap();

    match file_time.duration_since(time::SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
