use raylib::prelude::*;

const DEFAULT_FONT: &[u8; 712444] = include_bytes!("../fonts/NotoSerif-Regular.ttf");

fn main() {
    
    const APPLICATION_NAME: &str = "レイハ";
    const MAX_FPS: u32 = 16u32;

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .title(APPLICATION_NAME)
        .build();
    
    rl.set_target_fps(MAX_FPS);
    
    let fnt = rl.load_font_from_memory(&thread, ".ttf", DEFAULT_FONT, 64i32, None).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        //d.draw_text(format!("Behld... \n\n\nthe {}", APPLICATION_NAME).as_str(), 12, 12, 20, Color::BLACK);
        d.draw_text_ex(
            &fnt,
            format!("Behld... \n\n\nthe {}", APPLICATION_NAME).as_str(),
            Vector2::new(12f32, 12f32),
            20f32,
            0f32,
            Color::BLACK
        );
    }
    
    // fn load_font_from_system() -> Font {
        
    // }
}

