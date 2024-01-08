use color_print::cprintln;
use std::collections::HashMap;

pub fn print_hash(hash: HashMap<String, u64>) {
    cprintln!("<green><bold>RESULT</></>");

    for (key, value) in &hash {
        if key == "total" {
            cprintln!("<green><bold>{key}: {value}</bold></green>");
        } else {
            println!("{key}: {value}");
        }
    }
}
