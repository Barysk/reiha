use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

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

const VIRTUAL_SCREEN_SIZE: Vec2 = vec2(1600f32, 1200f32);

enum SlideType {
    Empty,
    Text,
    Image,
}

struct Theme {
    background_color: Color,
    font_color: Color,
}

const DARK_THEME: Theme = Theme {
    background_color: Color {
        r: 0f32,
        g: 0f32,
        b: 0f32,
        a: 255f32,
    },
    font_color: Color {
        r: 255f32,
        g: 255f32,
        b: 255f32,
        a: 255f32,
    },
};

const LIGHT_THEME: Theme = Theme {
    background_color: Color {
        r: 255f32,
        g: 255f32,
        b: 255f32,
        a: 255f32,
    },
    font_color: Color {
        r: 0f32,
        g: 0f32,
        b: 0f32,
        a: 255f32,
    },
};

struct Slide {
    num: u32,
    slide_type: SlideType,
    text: Option<String>,
    img: Option<Texture2D>,
    img_scale: Option<f32>,
    font_size: Option<u16>,
    comments: Option<String>,
}

impl Slide {
    fn new(
        num: u32,
        slide_type: SlideType,
        text: Option<String>,
        img: Option<Texture2D>,
        comments: Option<String>,
        virtual_screen_size: &Vec2,
        font: &Font,
    ) -> Self {
        let mut self_values = Self {
            num,
            slide_type,
            text: None,
            img: None,
            img_scale: None,
            font_size: None,
            comments,
        };

        match self_values.slide_type {
            SlideType::Empty => {}
            SlideType::Text => {
                self_values.font_size = Some(find_max_font_size(
                    text.as_ref().unwrap(),
                    Some(font),
                    1.0,
                    Some(1.0),
                    virtual_screen_size,
                ));
                self_values.text = text;
            }
            SlideType::Image => {
                let screen_height: f32 = virtual_screen_size.y;
                let screen_width: f32 = virtual_screen_size.x;
                self_values.img = img;

                let img: Texture2D = self_values.img.clone().unwrap();

                if img.height() > img.width() {
                    self_values.img_scale = Some(screen_height / img.height());
                } else {
                    self_values.img_scale = Some(screen_width / img.width());
                }
            }
        }

        self_values
    }

    fn draw(&self, font: &Font, font_color: &Color, virtual_screen_size: &Vec2) {
        match self.slide_type {
            SlideType::Empty => {}
            SlideType::Text => {
                draw_text_center(
                    &self.text.clone().unwrap(),
                    Some(font),
                    font_color,
                    virtual_screen_size,
                    self.font_size.unwrap_or(16u16),
                );
            }
            SlideType::Image => {
                draw_img_scaled_and_centered(
                    &self.img.clone().unwrap(),
                    &self.img_scale.clone().unwrap(),
                    virtual_screen_size,
                );
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
                if let Some(text) = &self.text {
                    for line in text.lines() {
                        println!("{}", line);
                    }
                }
            }
            SlideType::Image => {
                println!("(image)");
            }
        }

        if let Some(comments) = &self.comments {
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
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.contains(&"help".to_string()) {
        println!(
            "Usage: reiha <path>\n\
            Options:\n\
            -t dark|light|<bg_hex>x<font_hex> - Set theme\n\
            -l set texture filtering mode to linear, default is nearest\n\
            -f <font_path> - Use a custom font\n\
            -r <width>x<height> - Set virtual resolution (default 1600x1200) (max 3840x3840)\n\
            \n\
            bk 2025"
        );
        return;
    }

    // TODO: introduce .config/reiha/config
    // to set up defaults

    let input_path = &args[1];

    let mut theme = DARK_THEME;
    let mut filtering = FilterMode::Nearest;
    let mut font: Font = load_ttf_font_from_bytes(DEFAULT_FONT).unwrap();
    let mut virtual_screen_size = VIRTUAL_SCREEN_SIZE;

    for i in 2..args.len() {
        match args[i].as_str() {
            "-t" => {
                if let Some(value) = args.get(i + 1) {
                    if value == "dark" {
                        theme = DARK_THEME;
                    } else if value == "light" {
                        theme = LIGHT_THEME;
                    } else if value.contains('x') {
                        let parts: Vec<&str> = value.split('x').collect();
                        if parts.len() == 2 {
                            if let (Ok(bg), Ok(font)) =
                                (parse_hex_color(parts[0]), parse_hex_color(parts[1]))
                            {
                                theme = Theme {
                                    background_color: bg,
                                    font_color: font,
                                };
                            } else {
                                eprintln!("Invalid hex values in -t argument, using DARK_THEME");
                            }
                        } else {
                            eprintln!("Invalid format for -t argument, expected HEXxHEX");
                        }
                    } else {
                        eprintln!("Unknown theme option '{}', using DARK_THEME", value);
                    }
                }
            }
            "-l" => {
                filtering = FilterMode::Linear;
            }
            "-f" => {
                if let Some(path) = args.get(i + 1) {
                    let data = std::fs::read(path).expect("Failed to read font file");
                    font = load_ttf_font_from_bytes(&data).expect("Failed to load font");
                }
            }
            "-r" => {
                if let Some(value) = args.get(i + 1) {
                    if let Some((w, h)) = value.split_once('x') {
                        if let (Ok(w), Ok(h)) = (w.parse::<f32>(), h.parse::<f32>()) {
                            if w > 3840.0 || h > 3840.0 {
                                eprintln!(
                                    "Error: Resolution too large ({}x{}). Max allowed is 3840x3840.",
                                    w, h
                                );
                                std::process::exit(1);
                            }
                            virtual_screen_size = vec2(w, h);
                        } else {
                            eprintln!(
                                "Error: Invalid resolution format. Use <width>x<height> (e.g., 1600x1200)"
                            );
                            std::process::exit(1);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    set_default_filter_mode(filtering);
    println!("Filter set");

    let virtual_screen = Canvas2D::new(virtual_screen_size.x, virtual_screen_size.y);
    println!(
        "Virtual Screen created {}x{}",
        virtual_screen_size.x, virtual_screen_size.y
    );

    let mut is_fullscreen = false;

    let slides: Vec<Slide> = parse(input_path, &virtual_screen_size, &font).await;
    println!("Data parsed");

    let mut current_slide = 0;
    let mut sec_timer: f32 = 0f32;
    println!("Control vars created");

    let start_time = std::time::Instant::now();
    println!("Timestamp placed");

    let show_in_terminal = true;

    println!("Main loop start");
    loop {
        sec_timer -= get_frame_time();
        clear_background(BLACK);
        {
            set_camera(&virtual_screen.camera);
            clear_background(theme.background_color);

            if let Some(slide) = slides.get(current_slide) {
                slide.draw(&font, &theme.font_color, &virtual_screen_size);
                let elapsed = start_time.elapsed().as_secs();
                if show_in_terminal {
                    if sec_timer <= 0f32 {
                        clearscreen::clear().expect("failed to clear screen");
                        slide.print(slides.len());
                        print_time(Some(elapsed));
                        sec_timer = 1f32;
                    }
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

        if is_key_pressed(KeyCode::F) || is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        draw_fps();
        next_frame().await
    }
}

/// parses hex color to 255 format
fn parse_hex_color(s: &str) -> Result<Color, ()> {
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

async fn parse(path: &str, virtual_screen_size: &Vec2, font: &Font) -> Vec<Slide> {
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
                virtual_screen_size,
                font,
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
                virtual_screen_size,
                font,
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
                text_lines.push(line);
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
            virtual_screen_size,
            font,
        ));

        slide_num += 1;
    }

    slides
}

/// draws an image using draw_texture_ex
fn draw_img_scaled_and_centered(texture: &Texture2D, img_scale: &f32, virtual_screen_size: &Vec2) {
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

fn draw_text_center(
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

fn find_max_font_size(
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
    let step: u16 = match text_len {
        0..=3 => 72,
        4..=7 => 64,
        8..=15 => 16,
        _ => 4,
    };

    let mut font_size: u16 = 4u16;

    loop {
        let dim = measure_multiline_text(text, font, font_size, font_scale, line_distance_factor);

        if dim.width > target_width || dim.height > target_height {
            font_size -= step;
            debug_println!("decreased to {}", font_size);
            break;
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
