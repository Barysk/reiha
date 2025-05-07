use macroquad::prelude::*;

const DEFAULT_FONT: &[u8; 2697456] = include_bytes!("../fonts/monapo.ttf");
const APPLICATION_NAME: &str = "blah бла レイハ";

#[macroquad::main("BasicShapes")]
async fn main() {
    
    let fnt: Font = load_ttf_font_from_bytes(DEFAULT_FONT).unwrap();
    
    print!("{}",screen_width());
    print!("{}",screen_height());

    loop {
        clear_background(WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        draw_text_ex(
            APPLICATION_NAME,
            40.0,
            40.0,
            TextParams {
                font: Some(&fnt),
                font_size: 40u16,
                font_scale: 1f32,
                font_scale_aspect: 1f32,
                rotation: 0f32,
                color: BLACK,
            },
        );

        next_frame().await
    }
}
