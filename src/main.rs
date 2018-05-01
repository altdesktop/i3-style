use std::io::{BufReader, stdout};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

mod theme;
mod writer;

fn main() {
    let path = "./test-theme.yaml".to_string();
    let mut f = File::open(path).expect("file does not exist");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("could not read from file");

    let docs = YamlLoader::load_from_str(contents.as_str()).expect("could not parse yaml");
    let doc = &docs[0];

    let theme = theme::from_yaml(&doc);
    let mut f = File::open("/home/tony/.config/i3/config").expect("could not open file");
    let f = BufReader::new(f);

    writer::write_config(f, Option::None, theme);
}
