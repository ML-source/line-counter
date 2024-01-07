use indicatif::ProgressBar;
use std::collections::HashMap;
use crate::files::get_extension;

mod calc;

pub fn run(files: Vec<String>) -> HashMap<String, u64> {
  let mut line_stats: HashMap<String, u64> = HashMap::new();
  let mut total: u64 = 0;

  let bar = ProgressBar::new(files.len() as u64);

  for file in files {

    let extension = get_extension(&file);
    let lines = calc::count_lines(&file);
    total = lines + total;

    if line_stats.get(&extension) == None {
      line_stats.insert(extension, lines);
    } else {
      let old_count = line_stats.get(&extension).unwrap();
      let new_count = old_count + lines;
      line_stats.insert(extension, new_count);
    }

    bar.inc(1);
  }

  line_stats.insert("total".to_string(), total);

  bar.finish();
  line_stats
}
