use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

mod config_handle;
mod defaults;
mod slide;
mod theming;
mod utils;

use crate::config_handle::*;
use crate::defaults::*;
use crate::slide::*;
use crate::theming::*;
use crate::utils::*;

#[macroquad::main("Reiha")]
async fn main() {
    let config = Config::from_file();

    let mut theme = config.theme.unwrap_or(DARK_THEME);
    let mut filtering = config.filtering.unwrap_or(FilterMode::Nearest);
    let mut font: Font = if let Some(path) = &config.font_path {
        let data = std::fs::read(path).expect("Failed to read font file from config");
        load_ttf_font_from_bytes(&data).expect("Failed to load font from config")
    } else {
        load_ttf_font_from_bytes(DEFAULT_FONT).unwrap()
    };
    let mut virtual_screen_size = config.virtual_resolution.unwrap_or(VIRTUAL_SCREEN_SIZE);
    let mut numbering = config.numbering.unwrap_or(false);
    let mut preview = config.preview.unwrap_or(false);
    let mut numbering_anchor = config.numbering_anchor.unwrap_or(NumberingAnchor::BottomLeft);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.contains(&"help".to_string()) {
        println!(
            "Usage: reiha <path>\n\
            Options:\n\
            -t, --theme dark|light|<bg_hex>x<font_hex> - Set theme\n\
            -l, --linear - set texture filtering mode to linear, default is nearest\n\
            -f, --font <font_path> - Use a custom font\n\
            -r, --resolution <width>x<height> - Set virtual resolution (default 1600x1200) (max 3840x3840)\n\
            -n, --numbering - turn on the slide numbering\n\
            -a, --numbering_anchor - bl | bc | br | tl | tc | tr, if incorrect defaults to bl (bottom left)\n\
            -p, --preview - shows next slide in your terminal if there is such\n\
            ______________________\n\
            Reiha | ver1.2.2 | bk"
        );
        return;
    }

    let input_path = &args[1];

    for i in 2..args.len() {
        match args[i].as_str() {
            "-t" | "--theme" => {
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
            "-l" | "--linear" => {
                filtering = FilterMode::Linear;
            }
            "-f" | "--font" => {
                if let Some(path) = args.get(i + 1) {
                    let data = std::fs::read(path).expect("Failed to read font file");
                    font = load_ttf_font_from_bytes(&data).expect("Failed to load font");
                }
            }
            "-r" | "--resolution" => {
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
            "-n" | "--numbering" => {
                numbering = true;
            }
            "-a" | "--numbering_anchor" => {
                if let Some(val) = args.get(i + 1) {
                    match val.as_str() {
                        "bl" => numbering_anchor = NumberingAnchor::BottomLeft,
                        "bc" => numbering_anchor = NumberingAnchor::BottomCenter,
                        "br" => numbering_anchor = NumberingAnchor::BottomRight,
                        "tl" => numbering_anchor = NumberingAnchor::TopLeft,
                        "tc" => numbering_anchor = NumberingAnchor::TopCenter,
                        "tr" => numbering_anchor = NumberingAnchor::TopRight,
                        _    => numbering_anchor = NumberingAnchor::BottomLeft
                    }
                }
            }
            "-p" | "--preview" => {
                preview = true;
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

    let numbering_position: Vec2;
    match numbering_anchor {
        NumberingAnchor::BottomLeft => {
            numbering_position = Vec2 {
                x: virtual_screen_size.x / 400f32,
                y: virtual_screen_size.y - virtual_screen_size.y / 300f32,
            };
        },
        NumberingAnchor::BottomCenter => {
            numbering_position = Vec2 {
                x: virtual_screen_size.x / 2f32,
                y: virtual_screen_size.y - virtual_screen_size.y / 300f32,
            };
        },
        NumberingAnchor::BottomRight => {
            numbering_position = Vec2 {
                x: virtual_screen_size.x,
                y: virtual_screen_size.y - virtual_screen_size.y / 300f32,
            };
        },
        NumberingAnchor::TopLeft => {
            numbering_position = Vec2 {
                x: virtual_screen_size.x / 400f32,
                y: virtual_screen_size.y - virtual_screen_size.y,
            };
        },
        NumberingAnchor::TopCenter => {
            numbering_position = Vec2 {
                x: virtual_screen_size.x / 2f32,
                y: virtual_screen_size.y - virtual_screen_size.y,
            };
        },
        NumberingAnchor::TopRight => {
            numbering_position = Vec2 {
                x: virtual_screen_size.x,
                y: virtual_screen_size.y - virtual_screen_size.y,
            };
        },
    }

    let numbering_size = (virtual_screen_size.x as u16 / 32u16) as u16;

    println!("Main loop start");
    loop {
        sec_timer -= get_frame_time();
        clear_background(BLACK);
        {
            set_camera(&virtual_screen.camera);
            clear_background(theme.background_color);

            if let Some(slide) = slides.get(current_slide) {
                slide.draw(&font, &theme.font_color, &virtual_screen_size);
                if numbering {
                    draw_numbering(
                        &current_slide,
                        &font,
                        &numbering_position,
                        &numbering_size,
                        &theme,
                        &numbering_anchor,
                    );
                }
                let elapsed = start_time.elapsed().as_secs();
                if show_in_terminal {
                    if sec_timer <= 0f32 {
                        clear_screen();
                        if preview {
                            println!("< Current Slide >");
                        }
                        slide.print(slides.len());
                        print_time(Some(elapsed));
                        if preview {
                            if let Some(next_slide) = slides.get(current_slide + 1) {
                                println!("\n\n\n< Next Slide >");
                                next_slide.print(slides.len());
                            }
                        }

                        sec_timer = 1f32;
                    }
                }
            }

            set_default_camera();
        }
        virtual_screen.draw();

        // Inputs
        if is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::Down)
            || is_key_pressed(KeyCode::J)
            || is_key_pressed(KeyCode::L)
            || is_key_pressed(KeyCode::PageDown)
        {
            if current_slide < slides.len() - 1 {
                current_slide += 1;
                sec_timer = 0f32;
            }
        }

        if is_key_pressed(KeyCode::Left)
            || is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::K)
            || is_key_pressed(KeyCode::H)
            || is_key_pressed(KeyCode::PageUp)
        {
            if current_slide > 0 {
                current_slide -= 1;
                sec_timer = 0f32;
            }
        }

        if is_key_pressed(KeyCode::F) || is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        if is_key_pressed(KeyCode::S) {
            let new_theme = Theme{
                background_color: theme.font_color,
                font_color: theme.background_color
            };
            theme = new_theme;
        }

        if is_key_pressed(KeyCode::P) {
            preview = !preview;
            sec_timer = 0f32;
        }

        if is_key_pressed(KeyCode::N) {
            numbering = !numbering;
        }

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        //draw_fps();
        next_frame().await
    }
}
