use macroquad::prelude::*;
use regex::Regex;
use std::process::Command;

use crate::slide::*;

/// println that exists only in debug mod
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub async fn parse(path: &str, virtual_screen_size: &Vec2, font: &Font) -> Vec<Slide> {
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    let mut slides = Vec::new();
    let re = Regex::new(r"\n\s*\n+").unwrap();
    let mut paragraphs = re.split(&content);

    let mut slide_num = 1;

    while let Some(paragraph) = paragraphs.next() {
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
            ));
            slide_num += 1;
            continue;
        }

        // Image slide
        if lines[0].starts_with('@') {
            let img_path = lines[0][1..].trim();
            let texture = load_texture(img_path).await.expect("Failed to load image");

            let comments = lines
                .iter()
                .skip(1)
                .filter(|l| l.trim_start().starts_with('|'))
                .map(|l| *l)
                .collect::<Vec<&str>>()
                .join("\n");

            slides.push(Slide::new(
                slide_num,
                SlideType::Image,
                None,
                Some(texture),
                if comments.is_empty() {
                    None
                } else {
                    Some(comments)
                },
                virtual_screen_size,
                font,
            ));
            slide_num += 1;
            continue;
        }

        // Text slide
        let mut text_lines = Vec::new();
        let mut comment_lines = Vec::new();

        for line in lines {
            if line.trim_start().starts_with('|') {
                comment_lines.push(line);
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
        ));

        slide_num += 1;
    }

    slides
}

/// draws an image using draw_texture_ex
pub fn draw_img_scaled_and_centered(
    texture: &Texture2D,
    img_scale: &f32,
    virtual_screen_size: &Vec2,
) {
    let position: Vec2 = vec2(0f32, 0f32);
    let screen_center: Vec2 = vec2(virtual_screen_size.x / 2f32, virtual_screen_size.y / 2f32);

    let scaled_texture: Vec2 = vec2(texture.width() * img_scale, texture.height() * img_scale);
    let image_center: Vec2 = scaled_texture / 2f32;

    let corected_position: Vec2 = position + (screen_center - image_center);

    let dest_size: Vec2 = scaled_texture;

    draw_texture_ex(
        &texture,
        corected_position.x,
        corected_position.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            ..Default::default()
        },
    );
}

pub fn draw_text_center(
    text: &str,
    font: Option<&Font>,
    font_color: &Color,
    virtual_screen_size: &Vec2,
    font_size: u16,
) {
    let font_scale = 1f32;
    let font_scale_aspect = 1f32;
    let rotation = 0f32;
    let line_distance_factor = 1f32;

    let screen_center = vec2(virtual_screen_size.x / 2f32, virtual_screen_size.y / 2f32);
    let mut position = screen_center;

    // NOTE: Macroquad crate modifyed using this commit: https://github.com/not-fl3/macroquad/pull/884/files
    let text_dimentions = measure_multiline_text(
        text,
        font,
        font_size,
        font_scale,
        Some(line_distance_factor),
    );

    let text_center = vec2(
        position.x + (text_dimentions.width / 2f32),
        position.y + (text_dimentions.height / 2f32) - (font_size as f32 * 0.85), // 0.85 is a picked value that seems to be working
    );

    let difference = screen_center - text_center;
    position += difference;

    draw_multiline_text_ex(
        text,
        position.x,
        position.y,
        Some(line_distance_factor),
        TextParams {
            font,
            font_size,
            font_scale,
            font_scale_aspect,
            rotation,
            color: *font_color,
        },
    );
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
