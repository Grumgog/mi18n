use std::{path::PathBuf, fs, collections::{HashSet, HashMap}};

use serde_json::Value;

pub fn test_files(file_paths: &Vec<PathBuf>) {
    let mut key_super_set: HashSet<String> = HashSet::new();
    let mut file_set: HashMap<PathBuf, Box<HashSet<&String>>> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    for path in file_paths {
        if let Some(json_content) = read_file(path) {
            select_keys(&json_content, &mut keys);
            file_set.insert(path.to_path_buf(), Box::from(HashSet::from_iter(keys.clone().iter())));
            key_super_set.extend(keys.clone());
        }
    }
}

fn select_keys(json: &serde_json::Map<String, Value>, keys: &mut Vec<String>) {
    for (key, value) in json {
        keys.push(key.to_string());
        match value {
            Value::Object(obj) => select_keys(obj, keys),
            _ => {}
        }
    }
}

fn read_file(path: &PathBuf) -> Option<serde_json::Map<String, Value>> {
    match fs::read_to_string(path) {
        Ok(file_content) => {
            match serde_json::from_str(&file_content) {
                Ok(json_content) => return json_content,
                Err(_) => return None,
            }
        }        ,
        Err(_) => {
            return None;
        },
    }
}
