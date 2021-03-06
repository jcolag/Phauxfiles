use std::collections::HashMap;
use std::collections::hash_map::{Iter,Keys};

pub struct AbsUrl {
    pub path: String,
    args: HashMap<String, String>,
}

#[allow(dead_code)]
impl AbsUrl {
    pub fn new(path: &String) -> AbsUrl {
        let mut args: HashMap<String, String> = HashMap::new();
        let mut path_parts = path.split("?");
        let out_path = path_parts.next().unwrap().to_string();
        let query = match path_parts.next() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        for var in query.split("&") {
            let mut var_parts = var.split("=");
            let key = match var_parts.next() {
                Some(s) => s.to_string(),
                None => continue,
            };
            let value = match var_parts.next() {
                Some(s) => s.to_string(),
                None => "".to_string(),
            };
            if !key.is_empty() {
                args.insert(key, value);
            }
        }

        AbsUrl {
            path: out_path,
            args: args,
        }
    }

    pub fn get(&self, key: String, default: Option<String>) -> Option<String> {
        let keyslice = &*key;
        if self.args.contains_key(keyslice) {
            Some(self.args.get(keyslice).unwrap().to_string())
        } else {
            default
        }
    }

    pub fn keys(&self) -> Keys<String, String> {
        self.args.keys()
    }

    pub fn iter(&self) -> Iter<String, String> {
        self.args.iter()
    }
}
