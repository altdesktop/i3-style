use std::io::{BufReader, stdout};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

extern crate clap;
use clap::{Arg, App, SubCommand};

mod theme;
mod writer;

fn main() {
    let matches = App::new("i3-style")
        .version("1.0")
        .about("Make your i3 config a bit more stylish")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("file")
            .help("The config file the theme should be applied to. Defaults to the default i3 location.")
            .takes_value(true)
            )
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("file")
             .help("Apply the theme, attempt to validate the result, and write it to <file>")
             .takes_value(true)
            )
        .arg(Arg::with_name("save")
             .short("s")
             .long("save")
             .help("Set the output file to the path of the input file")
            )
        .arg(Arg::with_name("reload")
             .short("r")
             .long("reload")
             .help("Apply the theme by reloading the config")
            )
        .arg(Arg::with_name("list-all")
             .short("l")
             .long("list-all")
             .help("Print a list of all available themes")
            )
        .arg(Arg::with_name("to-theme")
             .short("t")
             .long("to-theme")
             .value_name("file")
             .help("Prints an i3-style theme based on the given config suitable for sharing with others")
             .takes_value(true)
            ).get_matches();
}

/*
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
*/
