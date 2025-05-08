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

#[macroquad::main("レイハ")]
async fn main() {
    
    let font: Font = load_ttf_font_from_bytes(DEFAULT_FONT).unwrap();
    println!("{}x{}",screen_width(),screen_height());
    println!("{}",get_time());

    let virtual_screen = Canvas2D::new(VIRTUAL_SCREEN.x, VIRTUAL_SCREEN.y);

    loop {
        clear_background(BLACK);
        {
            set_camera(&virtual_screen.camera);
            clear_background(GRAY);
            

            //draw_multiline_text_ex(text, x, y, line_distance_factor, params);
            draw_text_center("Multiline\ntext?\nis\nworks?", Some(&font));
            set_default_camera();
        }
        virtual_screen.draw();
        
        draw_fps();
        
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await
    }
    
    fn draw_text_center (text: &str, font: Option<&Font>) {
        // defaults
        let font_scale = 1f32;
        let font_scale_aspect = 1f32;
        let rotation = 0f32;
        let line_distance_factor = 1f32;

        // font size adatable
        let font_size = find_max_font_size(text, font, font_scale);
        
        // set position to center of the screen
        //let mut position = vec2(screen_width()/2f32, screen_height()/2f32);
        let mut position = vec2(VIRTUAL_SCREEN.x/2f32, VIRTUAL_SCREEN.y/2f32);
        
        //DEBUG
        draw_circle(position.x, position.y, 9f32, RED);
        
        // make true position
        // FIXME: measure_multiline_text should be introduced
        let text_dimentions = measure_text(text, font, font_size, font_scale);
        position.x -= text_dimentions.width / 2f32;
        position.y += text_dimentions.height / 2f32;

        //DEBUG
        draw_circle(position.x, position.y, 9f32, BLUE);

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
            }
        );
    }

    fn find_max_font_size(text: &str, font: Option<&Font>, font_scale: f32) -> u16 {
        let screen_w = VIRTUAL_SCREEN.y;
        let screen_h = VIRTUAL_SCREEN.x;
        let target_width = screen_w * 0.9;
        let target_height = screen_h * 0.9;

        let mut font_size = 16u16;
        let step = 16u16;

        loop {
            let dim = measure_text(text, font, font_size + step, font_scale);

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
    
}
