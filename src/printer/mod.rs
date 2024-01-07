use std::collections::HashMap;

pub fn print_hash(hash: HashMap<String, u64>) {
    for (key, value) in &hash {
        println!("{key}: {value}");
    }
}
