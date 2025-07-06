use macroquad::prelude::*;
use std::path::PathBuf;

use crate::theming::*;
use crate::utils::*;

pub struct Config {
    pub theme: Option<Theme>,
    pub filtering: Option<FilterMode>,
    pub font_path: Option<String>,
    pub virtual_resolution: Option<Vec2>,
    pub numbering: Option<bool>,
    pub preview: Option<bool>,
}

impl Config {
    pub fn from_file() -> Self {
        let mut config = Config {
            theme: None,
            filtering: None,
            font_path: None,
            virtual_resolution: None,
            numbering: None,
            preview: None,
        };

        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let config_path = home_dir.join(".config/reiha/config");

        if let Ok(lines) = std::fs::read_to_string(config_path) {
            let args = lines
                .lines()
                .flat_map(|line| line.split_whitespace())
                .map(str::to_string)
                .collect::<Vec<_>>();

            for i in 0..args.len() {
                match args[i].as_str() {
                    "-t" | "--theme" => {
                        if let Some(value) = args.get(i + 1) {
                            if value == "dark" {
                                config.theme = Some(DARK_THEME);
                            } else if value == "light" {
                                config.theme = Some(LIGHT_THEME);
                            } else if value.contains('x') {
                                let parts: Vec<&str> = value.split('x').collect();
                                if parts.len() == 2 {
                                    if let (Ok(bg), Ok(font)) =
                                        (parse_hex_color(parts[0]), parse_hex_color(parts[1]))
                                    {
                                        config.theme = Some(Theme {
                                            background_color: bg,
                                            font_color: font,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    "-l" | "--linear" => {
                        config.filtering = Some(FilterMode::Linear);
                    }
                    "-f" | "--font" => {
                        if let Some(path) = args.get(i + 1) {
                            config.font_path = Some(path.clone());
                        }
                    }
                    "-r" | "--resolution" => {
                        if let Some(value) = args.get(i + 1) {
                            if let Some((w, h)) = value.split_once('x') {
                                if let (Ok(w), Ok(h)) = (w.parse::<f32>(), h.parse::<f32>()) {
                                    if w <= 3840.0 && h <= 3840.0 {
                                        config.virtual_resolution = Some(vec2(w, h));
                                    }
                                }
                            }
                        }
                    }
                    "-n" | "--numbering" => {
                        config.numbering = Some(true);
                    }
                    "-p" | "--preview" => {
                        config.preview = Some(true);
                    }
                    _ => {}
                }
            }
        }

        config
    }
}
