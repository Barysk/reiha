use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

const DEFAULT_FONT: &[u8; 7835672] = include_bytes!("../fonts/ipaexm.ttf");
// TODO: Analyze later, introduce better font
/* use macroquad::prelude::*;

use macroquad_text::Fonts;

const NOTO_SANS: &[u8] = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");
const NOTO_SANS_JP: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Regular.otf");

fn window_conf() -> Conf { /* ommitted */ }

#[macroquad::main(window_conf)]
async fn main() {
  let mut fonts = Fonts::default();

  fonts.load_font_from_bytes("Noto Sans", NOTO_SANS).unwrap();
  fonts.load_font_from_bytes("Noto Sans JP", NOTO_SANS_JP).unwrap();

  loop {
    fonts.draw_text("Nice", 20.0, 0.0, 69, Color::from([1.0; 4]));
    fonts.draw_text("良い", 20.0, 89.0, 69, Color::from([1.0; 4]));
    fonts.draw_text("Nice 良い", 20.0, 178.0, 69, Color::from([1.0; 4]));

    next_frame().await;
  }
} */

//tmp
const TEST_FILE_PATH: &str = "test/input.txt";

const VIRTUAL_SCREEN: Vec2 = vec2(1600f32, 1200f32);

enum SlideType {
    Empty,
    Text,
    Image,
}

struct Slide {
    num: u32,
    slide_type: SlideType,
    text: Option<String>,
    img: Option<Texture2D>,
    font: Option<Font>,
    comments: Option<String>,
}

impl Slide {
    fn new(
        num: u32,
        slide_type: SlideType,
        text: Option<String>,
        img: Option<Texture2D>,
        comments: Option<String>,
    ) -> Self {
        let mut self_values = Self {
            num,
            slide_type,
            text: None,
            img: None,
            font: None,
            comments,
        };

        match self_values.slide_type {
            SlideType::Empty => {}
            SlideType::Text => {
                self_values.font = Some(load_ttf_font_from_bytes(DEFAULT_FONT).unwrap());
                self_values.text = text
            }
            SlideType::Image => {
                self_values.img = img;
            }
        }

        self_values
    }

    fn draw(&self) {
        match self.slide_type {
            SlideType::Empty => {}
            SlideType::Text => {
                draw_text_center(&self.text.clone().unwrap(), self.font.as_ref());
            }
            SlideType::Image => {
                draw_img_scaled_and_centered(&self.img.clone().unwrap());
            }
        }
    }

    fn print(&self, total: usize) {
        println!("========");

        match self.slide_type {
            SlideType::Empty => {
                println!("(empty slide)");
            }
            SlideType::Text => {
                if let Some(ref text) = self.text {
                    for line in text.lines() {
                        println!("{}", line);
                    }
                }
            }
            SlideType::Image => {
                println!("(image)");
            }
        }

        if let Some(ref comments) = self.comments {
            println!("--------");
            for line in comments.lines() {
                println!("| {}", line);
            }
        }

        println!("========");
        println!("[slide {}/{}]", self.num, total);
    }
}

fn print_time(elapsed_secs: Option<u64>) {
    if let Some(secs) = elapsed_secs {
        let minutes = secs / 60;
        let seconds = secs % 60;

        println!("[time {:01}:{:02}]", minutes, seconds);
    }
}

#[macroquad::main("レイハ")]
async fn main() {
    // NOTE: args
    // let args: Vec<String> = std::env::args().collect();
    // let input_path = args.get(1).expect("Usage: reiha <input-file>");

    // usage: reiha <path>
    // reiha help - to show this page
    //
    // -t dark/light - to set premade dark or light theme
    // -F nearest/linear - set filtering mode, default is ...
    // -f fontname - use custom font
    // -c #hex_background|#hex_font - set custom colors
    // -r 1600x1200 - to set resolution, default is 1600x1200

    set_default_filter_mode(FilterMode::Nearest);
    println!("Filter set");

    let virtual_screen = Canvas2D::new(VIRTUAL_SCREEN.x, VIRTUAL_SCREEN.y);
    println!("Virtual Screen created");

    // tmp
    let slides: Vec<Slide> = parse(TEST_FILE_PATH).await;
    // let slides: Vec<Slide> = parse(input_path).await;
    println!("Data parsed");

    let mut current_slide = 0;
    let mut sec_timer: f32 = 0f32;
    println!("Control vars created");

    let start_time = std::time::Instant::now();
    println!("Timestamp placed");

    println!("Main loop start");
    loop {
        sec_timer -= get_frame_time();
        clear_background(BLACK);
        {
            set_camera(&virtual_screen.camera);
            clear_background(GRAY);

            if let Some(slide) = slides.get(current_slide) {
                slide.draw();
                let elapsed = start_time.elapsed().as_secs();
                if sec_timer <= 0f32 {
                    clearscreen::clear().expect("failed to clear screen");
                    slide.print(slides.len());
                    print_time(Some(elapsed));
                    sec_timer = 1f32;
                }
            }

            set_default_camera();
        }
        virtual_screen.draw();

        // Inputs
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::PageDown) {
            if current_slide < slides.len() - 1 {
                current_slide += 1;
                sec_timer = 0f32;
            }
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::PageUp) {
            if current_slide > 0 {
                current_slide -= 1;
                sec_timer = 0f32;
            }
        }

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        draw_fps();
        next_frame().await
    }
}

// FIXME: Rveiew after getting soem sleep
async fn parse(path: &str) -> Vec<Slide> {
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    let mut slides = Vec::new();
    let mut paragraphs = content.split("\n\n");

    let mut slide_num = 1;

    while let Some(paragraph) = paragraphs.next() {
        let lines: Vec<&str> = paragraph.lines().collect();

        if lines.iter().all(|line| line.trim().is_empty()) {
            continue;
        }

        // Empty
        if lines[0].starts_with('\\') {
            let comments = lines
                .iter()
                .skip(1)
                .filter(|l| l.trim_start().starts_with('|'))
                .map(|l| l.trim_start_matches('|').trim())
                .collect::<Vec<&str>>()
                .join("\n");
            slides.push(Slide::new(
                slide_num,
                SlideType::Empty,
                None,
                None,
                Some(comments),
            ));
            slide_num += 1;
            continue;
        }

        // Image
        if lines[0].starts_with('@') {
            let img_path = lines[0][1..].trim();
            let texture = load_texture(img_path).await.expect("Failed to load image");

            let comments = lines
                .iter()
                .skip(1)
                .filter(|l| l.trim_start().starts_with('|'))
                .map(|l| l.trim_start_matches('|').trim())
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
            ));
            slide_num += 1;
            continue;
        }

        // Text
        let mut text_lines = Vec::new();
        let mut comment_lines = Vec::new();

        for line in lines {
            if line.trim_start().starts_with('|') {
                comment_lines.push(line.trim_start_matches('|').trim());
            } else {
                text_lines.push(line.trim());
            }
        }

        if text_lines.is_empty() && !comment_lines.is_empty() {
            continue;
        }

        // Text slide with optional comments
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
        ));

        slide_num += 1;
    }

    slides
}

/// draws an image using draw_texture_ex
fn draw_img_scaled_and_centered(texture: &Texture2D) {
    let position: Vec2 = vec2(0f32, 0f32);

    const SCREEN_CENTER: Vec2 = vec2(VIRTUAL_SCREEN.x / 2f32, VIRTUAL_SCREEN.y / 2f32);
    const SCREEN_HEIGHT: f32 = VIRTUAL_SCREEN.y;
    const SCREEN_WIDTH: f32 = VIRTUAL_SCREEN.x;

    let scale: f32;

    if texture.height() > texture.width() {
        scale = SCREEN_HEIGHT / texture.height();
    } else {
        scale = SCREEN_WIDTH / texture.width();
    }

    let scaled_texture: Vec2 = vec2(texture.width() * scale, texture.height() * scale);
    let image_center: Vec2 = scaled_texture / 2f32;

    let corected_position: Vec2 = position + (SCREEN_CENTER - image_center);

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

fn draw_text_center(text: &str, font: Option<&Font>) {
    let font_scale = 1f32;
    let font_scale_aspect = 1f32;
    let rotation = 0f32;
    let line_distance_factor = 1f32;

    let font_size = find_max_font_size(text, font, font_scale, Some(line_distance_factor));

    let screen_center = vec2(VIRTUAL_SCREEN.x / 2f32, VIRTUAL_SCREEN.y / 2f32);
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
        position.y + (text_dimentions.height / 2f32) - font_size as f32,
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
            color: WHITE,
        },
    );
}

// FIXME: font rendering is not a fast process
// take monospace glyph size for the probe
fn find_max_font_size(
    text: &str,
    font: Option<&Font>,
    font_scale: f32,
    line_distance_factor: Option<f32>,
) -> u16 {
    let screen_w = VIRTUAL_SCREEN.y;
    let screen_h = VIRTUAL_SCREEN.x;
    let target_width = screen_w * 0.95;
    let target_height = screen_h * 0.95;

    let mut font_size = 4u16;
    let step = 4u16;

    loop {
        let dim = measure_multiline_text(text, font, font_size, font_scale, line_distance_factor);

        if dim.width > target_width || dim.height > target_height {
            break;
        }

        font_size += step;

        if font_size >= 512 {
            break;
        }
    }

    font_size
}
