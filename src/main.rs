use std::io::{BufReader, stdout};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

mod theme;

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

    template_config(f, Option::None, theme);
}

fn leading_spaces(string: &String) -> String {
    let mut leading = String::new();

    for c in string.chars() {
        if c.is_whitespace() {
            leading.push(c);
        } else {
            break;
        }
    }

    leading
}

fn template_config(input: BufReader<File>, output: Option<String>, theme: theme::Theme) {
    let mut writer = match output {
        Some(x) => {
            let path = Path::new(x.as_str());
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(stdout()) as Box<Write>,
    };

    let mut in_bar = false;
    let mut in_colors = false;

    for line in input.lines() {
        let original_line = line.unwrap() + "\n";
        let leading = leading_spaces(&original_line);
        // TODO count leading spaces
        let line = original_line.trim();
        let mut vec: Vec<&str> = Vec::new();

        for word in line.split(' ') {
            if word.starts_with("#") {
                break;
            }
            if word != "" {
                vec.push(word);
            }
        }

        if vec.len() > 0 && !vec[0].starts_with("#") {
            if in_colors && vec[0] == "}" {
                in_colors = false;
                //writer.write(original_line.as_bytes());
                continue;
            } else if in_bar && vec[0] == "}" {
                in_bar = false;
                //writer.write(original_line.as_bytes());
                continue;
            }

            if in_colors {
                // TODO handle color block here
                if theme.bar_colors.is_none() {
                    //writer.write(original_line.as_bytes());
                    continue;
                }

                let bar_colors = &theme.bar_colors.as_ref().unwrap();
                let mut themed_vec = Vec::new();
                themed_vec.push(vec[0].clone());

                if vec!["separator", "background", "statusline"].contains(&vec[0]) {
                    themed_vec.push(match vec[0] {
                        "separator" => match bar_colors.separator {
                            Some(ref color) => color.as_str().clone(),
                            None => vec[1].clone()
                        },
                        "background" => match bar_colors.background {
                            Some(ref color) => color.as_str().clone(),
                            None => vec[1].clone()
                        },
                        "statusline" => match bar_colors.statusline {
                            Some(ref color) => color.as_str().clone(),
                            None => vec[1].clone()
                        },
                        _ => vec[1].clone()
                    });

                    //writer.write((leading + themed_vec.join(" ").as_str() + "\n").as_bytes());
                    continue;
                } else if vec!["focused_workspace", "active_workspace", "inactive_workspace", "urgent_workspace"].contains(&vec[0]) {
                    let group = match vec[0] {
                        "focused_workspace" => bar_colors.focused_workspace.as_ref(),
                        "active_workspace" => bar_colors.active_workspace.as_ref(),
                        "inactive_workspace" => bar_colors.inactive_workspace.as_ref(),
                        "urgent_workspace" => bar_colors.urgent_workspace.as_ref(),
                        _ => panic!("not reached")
                    };

                    if group.is_none() {
                        //writer.write(original_line.as_bytes());
                        continue;
                    }

                    let group = group.unwrap();
                    let mut themed_vec = Vec::new();
                    themed_vec.push(vec[0].clone());

                    let &border = match group.border.as_ref() {
                        Some(color) => color.clone(),
                        None => vec[1].to_string().clone()
                    };

                    themed_vec.push(border.as_str());

                    //themed_vec.push(group.border.unwrap_or(vec[1].clone().to_string()).as_str());
                    //themed_vec.push(group.background.unwrap_or(vec[2].clone().to_string()).as_str());
                    //themed_vec.push(group.text.unwrap_or(vec[3].clone().to_string()).as_str());
                    //themed_vec.push(group.text.unwrap_or(vec[4].clone().to_string()).as_str());
                    writer.write((leading + themed_vec.join(" ").as_str() + "\n").as_bytes());
                    continue;
                }
                continue;
            }

            if vec[0] == "bar" {
                in_bar = true;
                //writer.write(original_line.as_bytes());
                continue;
            }
            if in_bar && vec[0] == "colors" {
                in_colors = true;
                //writer.write(original_line.as_bytes());
                continue;
            }

            if vec[0].starts_with("client.") {
                // TODO handle window color block here
                continue;
            }

            //writer.write(original_line.as_bytes());
        }
    }
}
