use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use serde_json::Value;

pub fn test_files(file_paths: &Vec<PathBuf>) {
    let mut key_super_set: HashSet<String> = HashSet::new();
    let mut file_set: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    for path in file_paths {
        let mut keys: Vec<String> = Vec::new();
        if let Some(json_content) = read_file(path) {
            select_keys(&json_content, &mut keys, None);
            key_super_set.extend(keys.clone());
            let mut key_set = HashSet::<String>::new();
            key_set.extend(keys.clone());
            file_set.insert(path.to_path_buf(), key_set);
        }
    }

    for (path, set) in file_set {
        let diff = key_super_set.difference(&set);
        let f: Vec<&String> = diff.collect();
        if f.len() > 0 {
            println!("{} doesnt have next global keys:\n{:#?}", path.display(), f)
        } else {
            println!("{} is OK", path.display())
        }
    }
}

fn select_keys(
    json: &serde_json::Map<String, Value>,
    keys: &mut Vec<String>,
    parent_key: Option<String>,
) {
    for (key, value) in json {
        if let Some(ref parent_key_value) = parent_key {
            keys.push(format!("{}.{}", parent_key_value, key));
        } else {
            keys.push(key.to_string());
        }

        match value {
            Value::Object(obj) => {
                if let Some(ref parent_key_value) = parent_key {
                    select_keys(obj, keys, Some(format!("{}.{}", parent_key_value, key)))
                } else {
                    select_keys(obj, keys, Some(format!("{}", key)))
                }
            },
            _ => {}
        }
    }
}

fn read_file(path: &PathBuf) -> Option<serde_json::Map<String, Value>> {
    match fs::read_to_string(path) {
        Ok(file_content) => match serde_json::from_str(&file_content) {
            Ok(json_content) => return json_content,
            Err(_) => {
                eprintln!("Cannot read json from {}!", path.display());
                return None;
            }
        },
        Err(_) => {
            eprintln!("Cannot read {}!", path.display());
            return None;
        }
    }
}
