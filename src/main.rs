use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

#[derive(Debug)]
struct ColorGroup {
    border: Option<String>,
    background: Option<String>,
    text: Option<String>,
    indicator: Option<String>
}

impl ColorGroup {
    fn set_part(&mut self, part: &str, value: Option<String>) {
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
    focused: Option<ColorGroup>,
    focused_inactive: Option<ColorGroup>,
    unfocused: Option<ColorGroup>,
    urgent: Option<ColorGroup>
}

#[derive(Debug)]
struct BarColors {
    separator: Option<String>,
    background: Option<String>,
    statusline: Option<String>,
    focused_workspace: Option<ColorGroup>,
    active_workspace: Option<ColorGroup>,
    inactive_workspace: Option<ColorGroup>,
    urgent_workspace: Option<ColorGroup>
}

#[derive(Debug)]
struct Theme {
    window_colors: Option<WindowColors>,
    bar_colors: Option<BarColors>
}

fn parse_color(doc: &Yaml, color_spec: &Yaml) -> Option<String> {
    if color_spec.as_str().is_none() {
        return Option::None;
    }

    let color_spec = color_spec.as_str().unwrap();
    let colors = &doc["colors"];

    match colors[color_spec].as_str() {
        Some(color) => Option::from(color.to_string()),
        None => Option::from(color_spec.to_string())
    }
}

fn parse_color_group(doc: &Yaml, top_key: String, bottom_key: String) -> Option<ColorGroup> {
    let top_key = top_key.as_str();
    let bottom_key = bottom_key.as_str();

    if doc[top_key][bottom_key].as_hash().is_none() {
        return Option::None;
    }

    let group_hash = &doc[top_key][bottom_key];
    let colors = &doc["colors"];

    let mut group = ColorGroup {
        border: Option::None,
        background: Option::None,
        text: Option::None,
        indicator: Option::None
    };

    for &part in vec!["border", "background", "text", "indicator"].iter() {
        group.set_part(part, parse_color(&doc, &group_hash[part]));
    }

    Option::from(group)
}

fn parse_window_colors(doc: &Yaml) -> Option<WindowColors> {
    if doc["window_colors"].as_hash().is_none() {
        return Option::None;
    }

    Option::from(WindowColors {
        focused: parse_color_group(doc,
                          "window_colors".to_string(),
                          "focused".to_string()),
        focused_inactive: parse_color_group(doc,
                        "window_colors".to_string(),
                        "focused_inactive".to_string()),
        unfocused: parse_color_group(doc,
                        "window_colors".to_string(),
                        "unfocused".to_string()),
        urgent: parse_color_group(doc,
                        "window_colors".to_string(),
                        "urgent".to_string())
    })
}

fn parse_bar_colors(doc: &Yaml) -> Option<BarColors> {
    let bar_colors = &doc["bar_colors"];

    if bar_colors.as_hash().is_none() {
        return Option::None;
    }

    Option::from(BarColors {
        separator: parse_color(&doc, &bar_colors["separator"]),
        background: parse_color(&doc, &bar_colors["background"]),
        statusline: parse_color(&doc, &bar_colors["statusline"]),
        focused_workspace: parse_color_group(doc, "bar_colors".to_string(), "focused_workspace".to_string()),
        active_workspace: parse_color_group(doc, "bar_colors".to_string(), "active_workspace".to_string()),
        inactive_workspace: parse_color_group(doc, "bar_colors".to_string(), "inactive_workspace".to_string()),
        urgent_workspace: parse_color_group(doc, "bar_colors".to_string(), "urgent_workspace".to_string())
    })
}

fn load_theme(path: String) -> Theme {
    let mut f = File::open(path).expect("file does not exist");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("could not read from file");

    let docs = YamlLoader::load_from_str(contents.as_str()).expect("could not parse yaml");
    let doc = &docs[0];
    // TODO validate yaml

    Theme {
        window_colors: parse_window_colors(doc),
        bar_colors: parse_bar_colors(doc)
    }
}

fn main() {
    let theme = load_theme("./test-theme.yaml".to_string());
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
