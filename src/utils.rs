use macroquad::prelude::*;
use regex::Regex;
use std::process::Command;

use crate::slide::*;

/// println that exists only in debug mod
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub enum BackgroundMode {
    Fill,
    Fit
}

pub async fn parse(path: &str, virtual_screen_size: &Vec2, font: &Font, mono_font: &Font) -> Vec<Slide> {
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    let mut fixed = String::new();
    let mut in_code = false;

    for line in content.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("```") {
            in_code = !in_code;
            fixed.push_str(line);
            fixed.push('\n');
            continue;
        }

        if in_code {
            // replace truly empty lines inside code blocks to avoid splitting
            if trimmed.is_empty() {
                fixed.push_str("<!--EMPTY-CODE-LINE-->\n");
            } else {
                fixed.push_str(line);
                fixed.push('\n');
            }
        } else {
            fixed.push_str(line);
            fixed.push('\n');
        }
    }


    let mut slides = Vec::new();
    let re = Regex::new(r"\n\s*\n+").unwrap();
    let mut paragraphs = re.split(&fixed);
    // let mut paragraphs = re.split(&content);

    let mut slide_num = 1;

    while let Some(paragraph) = paragraphs.next() {
        let paragraph = paragraph.replace("<!--EMPTY-CODE-LINE-->", "");
        let lines: Vec<&str> = paragraph.lines().collect();

        if lines.iter().all(|line| line.trim().is_empty()) {
            continue;
        }

        // Empty slide
        if lines[0].starts_with('\\') {
            let comments = lines
                .iter()
                .skip(1)
                .filter(|l| l.trim_start().starts_with('|'))
                .map(|l| *l)
                .collect::<Vec<&str>>()
                .join("\n");

            slides.push(Slide::new(
                slide_num,
                SlideType::Empty,
                None,
                None,
                Some(comments),
                virtual_screen_size,
                font,
                mono_font,
            ));
            slide_num += 1;
            continue;
        }

        // Image slide
        if lines[0].starts_with('@') {
            let img_path = lines[0][1..].trim();
            let texture = load_texture(img_path).await.expect("Failed to load image");

            let mut text_lines = Vec::new();
            let mut comment_lines = Vec::new();

            for line in lines.iter().skip(1) {
                let l = line.trim_start();
                if l.starts_with('|') {
                    comment_lines.push(*line);
                } else if l.starts_with('~') {
                    text_lines.push("");
                } else if !l.is_empty() {
                    text_lines.push(*line);
                }
            }

            let slide_type:SlideType;
            if text_lines.is_empty() {
                slide_type = SlideType::Image;
            } else {
                slide_type = SlideType::TextImage;
            };

            slides.push(Slide::new(
                    slide_num,
                    slide_type,
                    if text_lines.is_empty() { None } else { Some(text_lines.join("\n")) },
                    Some(texture),
                    if comment_lines.is_empty() { None } else { Some(comment_lines.join("\n")) },
                    virtual_screen_size,
                    font,
                    mono_font,
            ));

            slide_num += 1;
            continue;
        }


        // Text slides
        let mut text_lines = Vec::new();
        let mut comment_lines = Vec::new();

        // Code
        if lines[0].starts_with("```") {
            let mut code_block_ended = false;
            for line in lines.iter().skip(1) {
                let line = line.trim_end();
                if line.starts_with("```") { code_block_ended = true; }
                if !code_block_ended { text_lines.push(line); }
                else { if line.starts_with('|') { comment_lines.push(line); }}
            }

            slides.push(Slide::new(
                    slide_num,
                    SlideType::Code,
                    Some(text_lines.join("\n")),
                    None,
                    if comment_lines.is_empty() {
                        None
                    } else {
                        Some(comment_lines.join("\n"))
                    },
                    virtual_screen_size,
                    font,
                    mono_font,
            ));

            slide_num += 1;
            continue;
        }


        // Text
        for line in lines {
            if line.trim_start().starts_with('|') {
                comment_lines.push(line);
            } else if line.trim_start().starts_with('~') {
                text_lines.push("");
            } else {
                text_lines.push(line);
            }
        }

        // if no text: it's source comment that is not going to be rendered anywhere
        if text_lines.is_empty() && !comment_lines.is_empty() {
            continue;
        }

        slides.push(Slide::new(
            slide_num,
            SlideType::Text,
            Some(text_lines.join("\n")),
            None,
            if comment_lines.is_empty() {
                None
            } else {
                Some(comment_lines.join("\n"))
            },
            virtual_screen_size,
            font,
            mono_font,
        ));

        slide_num += 1;
    }

    slides
}

pub fn find_max_font_size(
    text: &str,
    font: Option<&Font>,
    font_scale: f32,
    line_distance_factor: Option<f32>,
    virtual_screen_size: &Vec2,
) -> u16 {
    let screen_w = virtual_screen_size.x;
    let screen_h = virtual_screen_size.y;
    let target_width = screen_w * 0.96;
    let target_height = screen_h * 0.96;

    // Determine step size based on text length
    let text_len = text.chars().count();
    let mut step: u16 = match text_len {
        0..=3 => 96,
        4..=7 => 64,
        8..=15 => 48,
        _ => 32,
    };

    let mut depth = 4;

    let mut font_size: u16 = 4u16;

    loop {
        let dim = measure_multiline_text(text, font, font_size, font_scale, line_distance_factor);

        if dim.width > target_width || dim.height > target_height {
            font_size -= step;
            step = step / 2;
            depth -= 1;
            debug_println!("decreased to {}", font_size);
            if depth <= 0 {
                break;
            }
        }

        font_size += step;

        debug_println!("{}", font_size);

        if font_size >= 1024 {
            break;
        }
    }

    println!(".");

    font_size
}

/// parses hex color to Color
pub fn parse_hex_color(s: &str) -> Result<Color, ()> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 {
        return Err(());
    }
    if let Ok(rgb) = u32::from_str_radix(s, 16) {
        let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
        let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
        let b = (rgb & 0xFF) as f32 / 255.0;
        Ok(Color { r, g, b, a: 1.0 })
    } else {
        Err(())
    }
}

pub fn print_time(elapsed_secs: Option<u64>) {
    if let Some(secs) = elapsed_secs {
        let minutes = secs / 60;
        let seconds = secs % 60;

        println!("[time {:01}:{:02}]", minutes, seconds);
    }
}

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}
