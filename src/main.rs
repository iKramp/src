mod tokenizer;

fn tokenize_dir_recursively(path: &std::path::Path) -> Vec<(std::path::PathBuf, ())> {
    let mut result = Vec::new();
    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            result.extend(tokenize_dir_recursively(&path));
        }
    } else {
        //skip if not .rs
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            return result;
        }
        //print path
        println!("Tokenizing file: {}", path.display());
        let content = std::fs::read_to_string(path).unwrap();
        tokenizer::tokenize(content.as_str().chars().peekable(), path.to_str().unwrap());
        result.push((path.to_path_buf(), ()));
    }
    result
}

fn main() {
    let dir = std::env::args().nth(1).expect("Please provide a directory path");
    let _tokenized_files = tokenize_dir_recursively(std::path::Path::new(&dir));
}
