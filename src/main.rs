use raylib::prelude::*;

fn main() {
    
    const application_name: &str = "Present";
    const max_fps: u32 = 60u32;

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .title(application_name)
        .build();
    
    rl.set_target_fps(max_fps);
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        d.draw_text(format!("Behld... \n\n\nthe {}", application_name).as_str(), 12, 12, 20, Color::BLACK);
    }
}
