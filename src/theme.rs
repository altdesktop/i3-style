extern crate yaml_rust;

use yaml_rust::{Yaml};
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct ColorGroup {
    pub border: Option<String>,
    pub background: Option<String>,
    pub text: Option<String>,
    pub indicator: Option<String>
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

    fn empty() -> ColorGroup {
        ColorGroup {
            border: None,
            background: None,
            text: None,
            indicator: None
        }
    }
}

#[derive(Debug)]
pub struct WindowColors {
    pub focused: Option<ColorGroup>,
    pub focused_inactive: Option<ColorGroup>,
    pub unfocused: Option<ColorGroup>,
    pub urgent: Option<ColorGroup>
}

#[derive(Debug)]
pub struct BarColors {
    pub separator: Option<String>,
    pub background: Option<String>,
    pub statusline: Option<String>,
    pub focused_workspace: Option<ColorGroup>,
    pub active_workspace: Option<ColorGroup>,
    pub inactive_workspace: Option<ColorGroup>,
    pub urgent_workspace: Option<ColorGroup>
}

#[derive(Debug)]
pub struct Theme {
    pub description: Option<String>,
    pub window_colors: Option<WindowColors>,
    pub bar_colors: Option<BarColors>
}

impl Theme {
    fn ensure_window_colors(&mut self) {
        if self.window_colors.is_none() {
            self.window_colors = Some(WindowColors {
                focused: None,
                focused_inactive: None,
                unfocused: None,
                urgent: None
            });
        }
    }

    fn ensure_bar_colors(&mut self) {
        if self.bar_colors.is_none() {
            self.bar_colors = Some(BarColors {
                background: None,
                separator: None,
                statusline: None,
                active_workspace: None,
                focused_workspace: None,
                inactive_workspace: None,
                urgent_workspace: None
            });
        }
    }
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
        focused: parse_color_group(doc, "window_colors".to_string(), "focused".to_string()),
        focused_inactive: parse_color_group(doc, "window_colors".to_string(), "focused_inactive".to_string()),
        unfocused: parse_color_group(doc, "window_colors".to_string(), "unfocused".to_string()),
        urgent: parse_color_group(doc, "window_colors".to_string(), "urgent".to_string())
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

pub fn from_yaml(doc: &Yaml) -> Theme {
    let description = match doc["meta"]["description"].as_str() {
        Some(d) => Option::from(String::from(d)),
        None => Option::None
    };

    Theme {
        description: description,
        window_colors: parse_window_colors(doc),
        bar_colors: parse_bar_colors(doc)
    }
}

fn from_config_reader(input: BufReader<File>) -> Theme {
    let mut theme = Theme {
        description: Some("AUTOMATICALLY GENERATED THEME".to_string()),
        window_colors: None,
        bar_colors: None
    };

    let mut in_bar = false;
    let mut in_colors = false;

    for line in input.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut vec: Vec<&str> = Vec::new();

        for word in line.split(' ') {
            if word != "" {
                vec.push(word);
            }
        }

        if vec.len() == 0 || vec[0].starts_with("#") {
            continue;
        }

        if in_colors && vec[0] == "}" {
            in_colors = false;
            continue;
        } else if in_bar && vec[0] == "}" {
            in_bar = false;
            continue;
        } else if vec[0] == "bar" {
            in_bar = true;
            continue;
        } else if in_bar && vec[0] == "colors" {
            in_colors = true;
            continue;
        }

        if in_colors {
            match vec[0] {
                "separator" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    bar_colors.separator = Some(vec[1].to_string());
                    theme.bar_colors = Some(bar_colors);
                },
                "background" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    bar_colors.background = Some(vec[1].to_string());
                    theme.bar_colors = Some(bar_colors);
                },
                "statusline" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    bar_colors.statusline = Some(vec[1].to_string());
                    theme.bar_colors = Some(bar_colors);
                },
                "focused_workspace" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    let mut group = bar_colors.focused_workspace.unwrap_or(ColorGroup::empty());
                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    bar_colors.focused_workspace = Some(group);
                    theme.bar_colors = Some(bar_colors);
                },
                "inactive_workspace" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    let mut group = bar_colors.inactive_workspace.unwrap_or(ColorGroup::empty());
                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    bar_colors.inactive_workspace = Some(group);
                    theme.bar_colors = Some(bar_colors);
                },
                "urgent_workspace" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    let mut group = bar_colors.urgent_workspace.unwrap_or(ColorGroup::empty());
                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    bar_colors.urgent_workspace = Some(group);
                    theme.bar_colors = Some(bar_colors);
                },
                _ => ()
            };
        } else if !in_bar {
            match vec[0] {
                "client.focused" => {
                    theme.ensure_window_colors();
                    let mut window_colors = theme.window_colors.unwrap();
                    let mut group = window_colors.focused.unwrap_or(ColorGroup::empty());

                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    group.indicator = Some(vec[4].to_string());

                    window_colors.focused = Some(group);
                    theme.window_colors = Some(window_colors);
                },
                "client.focused_inactive" => {
                    theme.ensure_window_colors();
                    let mut window_colors = theme.window_colors.unwrap();
                    let mut group = window_colors.focused_inactive.unwrap_or(ColorGroup::empty());

                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    group.indicator = Some(vec[4].to_string());

                    window_colors.focused_inactive = Some(group);
                    theme.window_colors = Some(window_colors);

                },
                "client.unfocused" => {
                    theme.ensure_window_colors();
                    let mut window_colors = theme.window_colors.unwrap();
                    let mut group = window_colors.unfocused.unwrap_or(ColorGroup::empty());

                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    group.indicator = Some(vec[4].to_string());

                    window_colors.unfocused = Some(group);
                    theme.window_colors = Some(window_colors);

                },
                "client.urgent" => {
                    theme.ensure_window_colors();
                    let mut window_colors = theme.window_colors.unwrap();
                    let mut group = window_colors.urgent.unwrap_or(ColorGroup::empty());

                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    group.text = Some(vec[3].to_string());
                    group.indicator = Some(vec[4].to_string());

                    window_colors.urgent = Some(group);
                    theme.window_colors = Some(window_colors);

                },
                _ => ()
            };
        }
    }

    theme
}

pub fn from_config_file(input: &String) -> Theme {
    let input_file = File::open(input).unwrap();
    let reader = BufReader::new(input_file);
    from_config_reader(reader)
}
