#![allow(dead_code)]

mod item;
mod name;

use std::{collections::HashMap, path::Path};

use ast_trait::TokenIterator;
use tokenizer::tokenize_file;

pub struct Module {
    inner_attributes: (),
    items: (),
}

pub struct Crate {
    modules: HashMap<Box<[Box<str>]>, Module>,
}

pub fn parse_crate(root_path: &Path, root_file_name: &str) -> Crate {
    let mut modules = HashMap::new();

    let mut files_to_parse = Vec::new();
    files_to_parse.push(Path::new(root_file_name));

    while let Some(file) = files_to_parse.pop() {
        let mut full_file_path = root_path.to_owned();
        full_file_path.push(file);

        let tokens = tokenize_file(&full_file_path);
        let parsed = parse_token_stream(tokens.iter().peekable());
        // parse tokens to find mod declarations and add them to files_to_parse
        let k = file
            .iter()
            .map(|s| s.to_str().unwrap().into())
            .collect::<Box<[Box<str>]>>();
        modules.insert(k, parsed);
    }
    todo!()
}

pub fn parse_token_stream(tokens: TokenIterator) -> Module {
    todo!()
}
