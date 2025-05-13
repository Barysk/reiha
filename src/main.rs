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

const VIRTUAL_SCREEN: Vec2 = vec2(1600f32, 1200f32);

enum SlideType {
    Empty,
    Text,
    Image,
}

struct Slide {
    // id: u32,
    slide_type: SlideType,
    text: Option<String>,
    img: Option<Texture2D>,
    font: Option<Font>,
}

impl Slide {
    fn new(slide_type: SlideType, text: Option<String>, img: Option<Texture2D>) -> Self {
        let mut self_values = Self {
            slide_type,
            text: None,
            img: None,
            font: None,
        };

        match self_values.slide_type {
            SlideType::Empty => {}
            SlideType::Text => {
                self_values.font = Some(load_ttf_font_from_bytes(DEFAULT_FONT).unwrap());
                self_values.text = text;
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
                draw_texture(&self.img.clone().unwrap(), 0f32, 0f32, WHITE);
            }
        }
    }
}

#[macroquad::main("レイハ")]
async fn main() {
    //let font: Font = load_ttf_font_from_bytes(DEFAULT_FONT).unwrap();
    // println!("{}x{}", screen_width(), screen_height());
    // println!("{}", get_time());
    set_default_filter_mode(FilterMode::Nearest);
    println!("main loop started");

    let virtual_screen = Canvas2D::new(VIRTUAL_SCREEN.x, VIRTUAL_SCREEN.y);

    // Parse document
    let img_path: &str = "test/th.png";

    // Load all nessessary stuff here, becouse async is needed
    let img = load_texture(img_path).await.unwrap();
    let txt = "Based\nMultiline\nText".to_string();

    // create stuff from nessessary stuff
    let slide_img = Slide::new(SlideType::Image, None, Some(img));
    let slide_txt = Slide::new(SlideType::Text, Some(txt), None);

    loop {
        clear_background(BLACK);
        {
            set_camera(&virtual_screen.camera);
            clear_background(GRAY);

            // draw_text_center("Multiline text?\nworks?\nworks!", Some(&font));
            // draw_text_center("Based", Some(&font));

            slide_img.draw();

            set_default_camera();
        }
        virtual_screen.draw();

        draw_fps();

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await
    }
}

fn draw_text_center(text: &str, font: Option<&Font>) {
    let font_scale = 1f32;
    let font_scale_aspect = 1f32;
    let rotation = 0f32;
    let line_distance_factor = 1f32;

    let font_size = find_max_font_size(text, font, font_scale, Some(line_distance_factor));

    // set position to center of the screen
    let screen_center = vec2(VIRTUAL_SCREEN.x / 2f32, VIRTUAL_SCREEN.y / 2f32);
    let mut position = screen_center;

    // //DEBUG
    // draw_circle(position.x, position.y, 9f32, RED);

    // measure_multiline_text should be introduced
    // Someone already did it, so I took it
    // https://github.com/not-fl3/macroquad/pull/884/files
    let text_dimentions = measure_multiline_text(
        text,
        font,
        font_size,
        font_scale,
        Some(line_distance_factor),
    );

    // TODO:
    // if textCenter != VScreen center
    // adymi roznicu ad posiion

    let text_center = vec2(
        position.x + (text_dimentions.width / 2f32),
        position.y + (text_dimentions.height / 2f32) - font_size as f32,
    );

    let difference = screen_center - text_center;
    position += difference;

    // if text_center != screen_center {
    //     let difference = text_center - screen_center;
    //     position += difference;
    // }
    // position.x -= text_dimentions.width / 2f32;
    // position.y -= text_dimentions.height / 2f32;

    //position.x -= text_dimentions.width / 2f32;
    //position.y += text_dimentions.height / 2f32;

    //DEBUG
    draw_circle(position.x, position.y, 6f32, BLUE);

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

    let mut font_size = 16u16;
    let step = 16u16;

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
