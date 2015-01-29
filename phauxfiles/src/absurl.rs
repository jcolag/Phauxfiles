use std::collections::HashMap;

pub struct AbsUrl {
    pub path: String,
    args: HashMap<String, String>,
}

impl AbsUrl {
    pub fn new(path: &String) -> AbsUrl {
        let mut args: HashMap<String, String> = HashMap::new();
        let mut path_parts = path.split_str("?");
        let out_path = path_parts.next().unwrap().to_string();
        let query = match path_parts.next() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        for var in query.split_str("&") {
            let mut var_parts = var.split_str("=");
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

    pub fn get(&self, key: String, default: String) -> String {
        if self.args.contains_key(key.as_slice()) {
            self.args.get("count").unwrap().to_string()
        } else {
            default
        }
    }
}
