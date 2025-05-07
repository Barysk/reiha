use macroquad::prelude::*;

const DEFAULT_FONT: &[u8; 7835672] = include_bytes!("../fonts/ipaexm.ttf");

#[macroquad::main("レイハ")]
async fn main() {
    
    let fnt: Font = load_ttf_font_from_bytes(DEFAULT_FONT).unwrap();
    println!("{}x{}",screen_width(),screen_height());
    println!("{}",get_time());

    loop {
        clear_background(BLACK);

        draw_text_center("Test", Some(&fnt));

        draw_fps();
        next_frame().await
    }
}

fn draw_text_center (text: &str, font: Option<&Font>) {
    // defaults
    let font_scale = 1f32;
    let font_scale_aspect = 1f32;
    let rotation = 0f32;

    // font size adatable
    let font_size = find_max_font_size(text, font, font_scale);

    // set position to center of the screen
    let mut position = vec2(screen_width()/2f32, screen_height()/2f32);
    
    // make true position
    position.x -= measure_text(text, font, font_size, font_scale).width / 2f32;
    position.y += measure_text(text, font, font_size, font_scale).height / 2f32;

    draw_text_ex(
        text,
        position.x,
        position.y,
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

fn find_max_font_size(text: &str, font: Option<&Font>, font_scale: f32) -> u16 {
    let screen_w = screen_width();
    let screen_h = screen_height();
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