use std::io::{BufReader, stdout};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use theme::Theme;

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

pub fn write_config(input: &String, output: Option<&String>, theme: Theme) {
    let input_file = File::open(input).unwrap();
    let reader = BufReader::new(input_file);
    write_config_from_reader(reader, output, theme);
}

pub fn write_config_from_reader(input: BufReader<File>, output: Option<&String>, theme: Theme) {
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
            if word != "" {
                vec.push(word);
            }
        }

        if vec.len() > 0 && !vec[0].starts_with("#") {
            if in_colors && vec[0] == "}" {
                in_colors = false;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            } else if in_bar && vec[0] == "}" {
                in_bar = false;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            }

            if in_colors {
                // TODO handle color block here
                if theme.bar_colors.is_none() {
                    writer.write(original_line.as_bytes()).unwrap();
                    continue;
                }

                let bar_colors = &theme.bar_colors.as_ref().unwrap();

                if vec!["separator", "background", "statusline"].contains(&vec[0]) {
                    writer.write(leading.as_bytes()).unwrap();
                    writer.write(vec[0].as_bytes()).unwrap();
                    writer.write(b" ").unwrap();

                    writer.write(match vec[0] {
                        "separator" => match bar_colors.separator {
                            Some(ref color) => color.as_bytes(),
                            None => vec[1].as_bytes()
                        },
                        "background" => match bar_colors.background {
                            Some(ref color) => color.as_bytes(),
                            None => vec[1].as_bytes(),
                        },
                        "statusline" => match bar_colors.statusline {
                            Some(ref color) => color.as_bytes(),
                            None => vec[1].as_bytes(),
                        },
                        _ => vec[1].as_bytes(),
                    }).unwrap();
                    writer.write(b"\n").unwrap();
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
                        writer.write(original_line.as_bytes()).unwrap();
                        continue;
                    }

                    let group = group.unwrap();

                    writer.write(leading.as_bytes()).unwrap();
                    writer.write(vec[0].as_bytes()).unwrap();
                    writer.write(b" ").unwrap();

                    writer.write(match group.border.as_ref() {
                        Some(color) => color.as_bytes(),
                        None => vec[1].as_bytes()
                    }).unwrap();
                    writer.write(b" ").unwrap();

                    writer.write(match group.background.as_ref() {
                        Some(color) => color.as_bytes(),
                        None => vec[2].as_bytes()
                    }).unwrap();
                    writer.write(b" ").unwrap();

                    writer.write(match group.text.as_ref() {
                        Some(color) => color.as_bytes(),
                        None => vec[3].as_bytes()
                    }).unwrap();
                    writer.write(b" ").unwrap();

                    writer.write(match group.indicator.as_ref() {
                        Some(color) => color.as_bytes(),
                        None => vec[3].as_bytes()
                    }).unwrap();
                    writer.write(b"\n").unwrap();

                    continue;
                }
                continue;
            }

            if vec[0] == "bar" {
                in_bar = true;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            }
            if in_bar && vec[0] == "colors" {
                in_colors = true;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            }

            if vec!["client.focused", "client.unfocused", "client.focused_inactive", "client.urgent"].contains(&vec[0]) {
                if theme.window_colors.is_none() {
                    writer.write(original_line.as_bytes()).unwrap();
                    continue;
                }

                let window_colors = &theme.window_colors.as_ref().unwrap();

                let group = match vec[0] {
                    "client.focused" => window_colors.focused.as_ref(),
                    "client.unfocused" => window_colors.unfocused.as_ref(),
                    "client.focused_inactive" => window_colors.focused_inactive.as_ref(),
                    "client.urgent" => window_colors.urgent.as_ref(),
                    _ => panic!("not reached")
                };

                if group.is_none() {
                    writer.write(original_line.as_bytes()).unwrap();
                    continue;
                }

                let group = group.unwrap();

                writer.write(leading.as_bytes()).unwrap();
                writer.write(vec[0].as_bytes()).unwrap();
                writer.write(b" ").unwrap();

                writer.write(match group.border.as_ref() {
                    Some(color) => color.as_bytes(),
                    None => vec[1].as_bytes()
                }).unwrap();
                writer.write(b" ").unwrap();

                writer.write(match group.background.as_ref() {
                    Some(color) => color.as_bytes(),
                    None => vec[2].as_bytes()
                }).unwrap();
                writer.write(b" ").unwrap();

                writer.write(match group.text.as_ref() {
                    Some(color) => color.as_bytes(),
                    None => vec[3].as_bytes()
                }).unwrap();
                writer.write(b" ").unwrap();

                writer.write(match group.indicator.as_ref() {
                    Some(color) => color.as_bytes(),
                    None => vec[3].as_bytes()
                }).unwrap();
                writer.write(b" ").unwrap();

                writer.write(b"\n").unwrap();
                continue;
            }
        }

        writer.write(original_line.as_bytes()).unwrap();
    }
}
