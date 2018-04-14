use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

#[derive(Debug)]
struct ColorSpec {
    border: Option<String>,
    background: Option<String>,
    text: Option<String>,
    indicator: Option<String>
}

impl ColorSpec {
    fn set_part(&mut self, part: &str, value: &str) {
        let value = Option::from(value.to_string());

        match part {
            "border" => self.border = value,
            "background" => self.background = value,
            "text" => self.text = value,
            "indicator" => self.indicator = value,
            _ => panic!("got unknown part: {}", part)
        }
    }
}

#[derive(Debug)]
struct WindowColors {
    focused: Option<ColorSpec>,
    focused_inactive: Option<ColorSpec>,
    unfocused: Option<ColorSpec>,
    urgent: Option<ColorSpec>
}

#[derive(Debug)]
struct Theme {
    window_colors: WindowColors,
}

fn parse_color_spec(doc: &Yaml, top_key: String, bottom_key: String) -> Option<ColorSpec> {
	let top_key = top_key.as_str();
	let bottom_key = bottom_key.as_str();

	if doc[top_key][bottom_key].as_hash().is_none() {
		return Option::None;
	}

	let spec_hash = &doc[top_key][bottom_key];
	let colors = &doc["colors"];

    let mut spec = ColorSpec {
        border: Option::None,
        background: Option::None,
        text: Option::None,
        indicator: Option::None
    };

    for &part in vec!["border", "background", "text", "indicator"].iter() {
        if spec_hash[part].as_str().is_none() {
            continue;
        }

        let part_val = spec_hash[part].as_str().unwrap();

        match doc["colors"][part_val].as_str() {
            Some(color) => {
                spec.set_part(part, color);
            },
            None => {
                spec.set_part(part, part_val);
            }
        }
    }

    Option::from(spec)
}

fn load_theme(path: String) -> Theme {
    let mut f = File::open(path).expect("file does not exist");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("could not read from file");

    let docs = YamlLoader::load_from_str(contents.as_str()).expect("could not parse yaml");
    let doc = &docs[0];
    // TODO validate yaml

    let colors = doc["colors"].as_hash().unwrap();

    let window_colors = WindowColors {
        focused: parse_color_spec(doc,
                          "window_colors".to_string(),
                          "focused".to_string()),
        focused_inactive: parse_color_spec(doc,
                        "window_colors".to_string(),
                        "focused_inactive".to_string()),
        unfocused: parse_color_spec(doc,
                        "window_colors".to_string(),
                        "unfocused".to_string()),
        urgent: parse_color_spec(doc,
                        "window_colors".to_string(),
                        "urgent".to_string())
    };

    Theme {
        window_colors: window_colors
    }
}

fn main() {
    let theme = load_theme("./test-theme.yaml".to_string());
    println!("{:?}", theme);
}

fn template_config(path: String) {
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
