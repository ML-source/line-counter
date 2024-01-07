use walkdir::WalkDir;

pub fn get_files(path: &String) -> Vec<String> {
  let mut result = Vec::new();

  for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
      let is_hidden = entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false);

      if entry.file_type().is_file() && !is_hidden {
          result.push(entry.path().display().to_string());

      }
  }

  result
}

pub fn get_extension(path: &String) -> String {
    let file = get_file(&path).expect("error");

    if !file.contains('.') {
        return "file without extension".to_string();
    }

    let parts: Vec<&str> = file.split('.').collect();

    match parts.last() {
        Some(el) => el.to_string(),
        None => "could not determine file extension".to_string()
    }
}

fn get_file(path: &String) -> Option<String> {
    let parts: Vec<&str> = path.split('/').collect();

    match parts.last() {
      Some(el) => Some(el.to_string()),
      None => None
    }
}
