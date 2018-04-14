use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

mod theme;


fn main() {
    let path = "./test-theme.yaml".to_string();
    let mut f = File::open(path).expect("file does not exist");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("could not read from file");

    let docs = YamlLoader::load_from_str(contents.as_str()).expect("could not parse yaml");
    let doc = &docs[0];

    let theme = theme::from_yaml(&doc);
    println!("{:?}", theme);
}

fn template_config(path: String) {
    /*
    let out_writer = match out {
        Some(x) => {
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(io::stdout()) as Box<Write>,
    };
   */
    let mut f = File::open("/home/tony/.config/i3/config").expect("could not open file");
    let f = BufReader::new(f);

    let mut in_bar = false;
    let mut in_colors = false;

    for line in f.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut vec: Vec<&str> = Vec::new();

        for word in line.split(' ') {
            if word != "" {
                vec.push(word);
            }
        }

        if vec.len() > 0 && !vec[0].starts_with("#") {
            if in_colors && vec[0] == "}" {
                in_colors = false;
                continue;
            } else if in_bar && vec[0] == "}" {
                in_bar = false;
                continue;
            }

            if in_colors {
                // TODO handle color block here
                //println!("{:?}", line);
                continue;
            }

            if vec[0] == "bar" {
                in_bar = true;
                continue;
            }
            if in_bar && vec[0] == "colors" {
                in_colors = true;
                continue;
            }

            if line.starts_with("client.") {
                // TODO handle client line
                println!("{:?}", line);
            }
        }
    }
}
