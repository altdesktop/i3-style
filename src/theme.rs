extern crate yaml_rust;
extern crate colornamer;
extern crate regex;
extern crate linked_hash_map;

use yaml_rust::Yaml;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use linked_hash_map::LinkedHashMap;

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

    fn to_yaml(&self, color_map: &ColorMap) -> Yaml {
        let mut group_yaml: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        if self.border.is_some() {
            group_yaml.insert(Yaml::String("border".to_string()), Yaml::String(color_map.get_color(&self.border.as_ref().unwrap().to_string())));
        }
        if self.background.is_some() {
            group_yaml.insert(Yaml::String("background".to_string()), Yaml::String(color_map.get_color(&self.background.as_ref().unwrap().to_string())));
        }
        if self.text.is_some() {
            group_yaml.insert(Yaml::String("text".to_string()), Yaml::String(color_map.get_color(&self.text.as_ref().unwrap().to_string())));
        }
        if self.indicator.is_some() {
            group_yaml.insert(Yaml::String("indicator".to_string()), Yaml::String(color_map.get_color(&self.indicator.as_ref().unwrap().to_string())));
        }
        Yaml::Hash(group_yaml)
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

#[derive(Debug)]
struct ColorMap {
    colors: LinkedHashMap<String, String>
}

impl ColorMap {
    pub fn new() -> ColorMap {
        ColorMap {
            colors: LinkedHashMap::new()
        }
    }

    fn has_color(&self, hex: &String) -> bool {
        for (_key, value) in &self.colors {
            if hex == value {
                return true;
            }
        }
        return false;
    }

    fn get_color(&self, hex: &String) -> String {
        let hex_uc = hex.to_uppercase();
        for (key, value) in &self.colors {
            if &hex_uc == value {
                return key.to_string();
            }
        }
        return hex.to_string();
    }

    fn add_hex(&mut self, hex: &Option<String>) {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"-(\d+)$").unwrap();
        }

        match hex {
            &Some(ref h) => {
                let h = h.to_uppercase();
                if self.has_color(&h) {
                    return;
                }

                let mut color_name = colornamer::name_color_hex(h.as_str(), colornamer::Colors::HTML);
                let ref mut colors = self.colors;
                while colors.contains_key(&color_name) {
                    if !RE.is_match(&color_name) {
                        color_name = color_name + "-1";
                    } else {
                        let cpy = color_name.clone();
                        let captures = RE.captures(cpy.as_str()).unwrap().get(1).unwrap();
                        let num: String = color_name.chars().skip(captures.start()).collect();
                        let num: u32 = num.parse().unwrap();
                        let num = num + 1;
                        color_name = color_name.chars().take(captures.start()).collect();
                        color_name = color_name + num.to_string().as_str();
                    }
                }
                colors.insert(color_name.to_string(), h.to_string());
            },
            &None => ()
        }
    }

    fn add_color_group(&mut self, group: &Option<ColorGroup>) {
        match group {
            &Some(ref g) => {
                self.add_hex(&g.border);
                self.add_hex(&g.background);
                self.add_hex(&g.text);
                self.add_hex(&g.indicator);
            },
            &None => ()
        }
    }
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

    pub fn to_yaml_with_colors(self) -> Yaml {
        let mut colormap = ColorMap::new();

        let ref bar_colors = self.bar_colors;
        match bar_colors {
            &Some(ref bc) => {
                colormap.add_hex(&bc.separator);
                colormap.add_hex(&bc.background);
                colormap.add_hex(&bc.statusline);
                colormap.add_color_group(&bc.focused_workspace);
                colormap.add_color_group(&bc.active_workspace);
                colormap.add_color_group(&bc.inactive_workspace);
                colormap.add_color_group(&bc.urgent_workspace);
            },
            &None => ()
        }
        let ref window_colors = self.window_colors;
        match window_colors {
            &Some(ref wc) => {
                colormap.add_color_group(&wc.focused);
                colormap.add_color_group(&wc.focused_inactive);
                colormap.add_color_group(&wc.unfocused);
                colormap.add_color_group(&wc.urgent);
            },
            &None => ()
        }

        let mut toplevel_yaml: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        let mut colormap_yaml: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        let mut metamap_yaml: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        let mut window_colors_yaml: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        let mut bar_colors_yaml: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();

        metamap_yaml.insert(Yaml::String("description".to_string()), Yaml::String(self.description.unwrap()));

        for (key, value) in &colormap.colors {
            colormap_yaml.insert(Yaml::String(key.to_string()), Yaml::String(value.to_string()));
        }

        match window_colors {
            &Some(ref wc) => {
                if wc.focused.is_some() {
                    window_colors_yaml.insert(Yaml::String("focused".to_string()), wc.focused.as_ref().unwrap().to_yaml(&colormap));
                }
                if wc.focused_inactive.is_some() {
                    window_colors_yaml.insert(Yaml::String("focused_inactive".to_string()), wc.focused_inactive.as_ref().unwrap().to_yaml(&colormap));
                }
                if wc.unfocused.is_some() {
                    window_colors_yaml.insert(Yaml::String("unfocused".to_string()), wc.unfocused.as_ref().unwrap().to_yaml(&colormap));
                }
                if wc.urgent.is_some() {
                    window_colors_yaml.insert(Yaml::String("urgent".to_string()), wc.urgent.as_ref().unwrap().to_yaml(&colormap));
                }
            },
            &None => ()
        }

        match bar_colors {
            &Some(ref bc) => {
                if bc.background.is_some() {
                    bar_colors_yaml.insert(Yaml::String("background".to_string()), Yaml::String(colormap.get_color(&bc.background.as_ref().unwrap().to_string())));
                }
                if bc.statusline.is_some() {
                    bar_colors_yaml.insert(Yaml::String("statusline".to_string()), Yaml::String(colormap.get_color(&bc.statusline.as_ref().unwrap().to_string())));
                }
                if bc.separator.is_some() {
                    bar_colors_yaml.insert(Yaml::String("separator".to_string()), Yaml::String(colormap.get_color(&bc.separator.as_ref().unwrap().to_string())));
                }
                if bc.focused_workspace.is_some() {
                    bar_colors_yaml.insert(Yaml::String("focused_workspace".to_string()), bc.focused_workspace.as_ref().unwrap().to_yaml(&colormap));
                }
                if bc.active_workspace.is_some() {
                    bar_colors_yaml.insert(Yaml::String("active_workspace".to_string()), bc.active_workspace.as_ref().unwrap().to_yaml(&colormap));
                }
                if bc.urgent_workspace.is_some() {
                    bar_colors_yaml.insert(Yaml::String("urgent_workspace".to_string()), bc.urgent_workspace.as_ref().unwrap().to_yaml(&colormap));
                }
            },
            &None => ()
        }

        toplevel_yaml.insert(Yaml::String("meta".to_string()), Yaml::Hash(metamap_yaml));
        toplevel_yaml.insert(Yaml::String("colors".to_string()), Yaml::Hash(colormap_yaml));
        toplevel_yaml.insert(Yaml::String("window_colors".to_string()), Yaml::Hash(window_colors_yaml));
        toplevel_yaml.insert(Yaml::String("bar_colors".to_string()), Yaml::Hash(bar_colors_yaml));

        Yaml::Hash(toplevel_yaml)
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
                    if vec.get(3).is_some() {
                        group.text = Some(vec[3].to_string());
                    }
                    bar_colors.focused_workspace = Some(group);
                    theme.bar_colors = Some(bar_colors);
                },
                "inactive_workspace" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    let mut group = bar_colors.inactive_workspace.unwrap_or(ColorGroup::empty());
                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    if vec.get(3).is_some() {
                        group.text = Some(vec[3].to_string());
                    }
                    bar_colors.inactive_workspace = Some(group);
                    theme.bar_colors = Some(bar_colors);
                },
                "urgent_workspace" => {
                    theme.ensure_bar_colors();
                    let mut bar_colors = theme.bar_colors.unwrap();
                    let mut group = bar_colors.urgent_workspace.unwrap_or(ColorGroup::empty());
                    group.border = Some(vec[1].to_string());
                    group.background = Some(vec[2].to_string());
                    if vec.get(3).is_some() {
                        group.text = Some(vec[3].to_string());
                    }
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
                    if vec.get(4).is_some() {
                        group.indicator = Some(vec[4].to_string());
                    }

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
                    if vec.get(4).is_some() {
                        group.indicator = Some(vec[4].to_string());
                    }

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
                    if vec.get(4).is_some() {
                        group.indicator = Some(vec[4].to_string());
                    }

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
                    if vec.get(4).is_some() {
                        group.indicator = Some(vec[4].to_string());
                    }

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

#[cfg(test)]
mod tests {
    extern crate tempfile;
    use super::*;

    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    pub fn test_config_parsing() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test-resources/minimal-config");
        let theme = from_config_file(&d.to_str().unwrap().to_string());

        assert_eq!(theme.description.unwrap(), "AUTOMATICALLY GENERATED THEME");

        let wc = theme.window_colors.unwrap();
        let bc = theme.bar_colors.unwrap();

        let group = wc.focused.unwrap();
        assert_eq!(group.border, Some("#859900".to_string()));
        assert_eq!(group.background, Some("#859900".to_string()));
        assert_eq!(group.text, Some("#fdf6e3".to_string()));
        assert_eq!(group.indicator, None);

        let group = wc.focused_inactive.unwrap();
        assert_eq!(group.border, Some("#073642".to_string()));
        assert_eq!(group.background, Some("#073642".to_string()));
        assert_eq!(group.text, Some("#eee8d5".to_string()));
        assert_eq!(group.indicator, None);

        let group = wc.unfocused.unwrap();
        assert_eq!(group.border, Some("#073642".to_string()));
        assert_eq!(group.background, Some("#073642".to_string()));
        assert_eq!(group.text, Some("#93a1a1".to_string()));
        assert_eq!(group.indicator, None);

        let group = wc.urgent.unwrap();
        assert_eq!(group.border, Some("#d33682".to_string()));
        assert_eq!(group.background, Some("#d33682".to_string()));
        assert_eq!(group.text, Some("#fdf6e3".to_string()));
        assert_eq!(group.indicator, None);

        // TODO finish these tests
    }
}
