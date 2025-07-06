use macroquad::prelude::*;

use crate::theming::*;
use crate::utils::*;

pub enum SlideType {
    Empty,
    Text,
    Image,
}

pub struct Slide {
    pub num: u32,
    pub slide_type: SlideType,
    pub text: Option<String>,
    pub img: Option<Texture2D>,
    pub img_scale: Option<f32>,
    pub font_size: Option<u16>,
    pub comments: Option<String>,
}

impl Slide {
    pub fn new(
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

                if let Some(ref image) = self_values.img {
                    let scale_x = screen_width / image.width();
                    let scale_y = screen_height / image.height();
                    self_values.img_scale = Some(scale_x.min(scale_y));
                }
            }
        }

        self_values
    }

    pub fn draw(&self, font: &Font, font_color: &Color, virtual_screen_size: &Vec2) {
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

    pub fn print(&self, total: usize) {
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
                println!("{}", line);
            }
        }

        println!("========");
        println!("[slide {}/{}]", self.num, total);
    }
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

pub fn draw_numbering(
    current_slide: &usize,
    font: &Font,
    numbering_position: &Vec2,
    numbering_offset: &f32,
    numbering_size: &u16,
    theme: &Theme,
) {
    draw_text_ex(
        &(current_slide + 1).to_string(),
        numbering_position.x - numbering_offset,
        numbering_position.y - numbering_offset,
        TextParams {
            font: Some(&font),
            font_size: *numbering_size,
            font_scale: 1f32,
            font_scale_aspect: 1f32,
            rotation: 0f32,
            color: theme.background_color,
        },
    );
    draw_text_ex(
        &(current_slide + 1).to_string(),
        numbering_position.x - numbering_offset,
        numbering_position.y + numbering_offset,
        TextParams {
            font: Some(&font),
            font_size: *numbering_size,
            font_scale: 1f32,
            font_scale_aspect: 1f32,
            rotation: 0f32,
            color: theme.background_color,
        },
    );
    draw_text_ex(
        &(current_slide + 1).to_string(),
        numbering_position.x + numbering_offset,
        numbering_position.y - numbering_offset,
        TextParams {
            font: Some(&font),
            font_size: *numbering_size,
            font_scale: 1f32,
            font_scale_aspect: 1f32,
            rotation: 0f32,
            color: theme.background_color,
        },
    );
    draw_text_ex(
        &(current_slide + 1).to_string(),
        numbering_position.x + numbering_offset,
        numbering_position.y + numbering_offset,
        TextParams {
            font: Some(&font),
            font_size: *numbering_size,
            font_scale: 1f32,
            font_scale_aspect: 1f32,
            rotation: 0f32,
            color: theme.background_color,
        },
    );
    draw_text_ex(
        &(current_slide + 1).to_string(),
        numbering_position.x,
        numbering_position.y,
        TextParams {
            font: Some(&font),
            font_size: *numbering_size,
            font_scale: 1f32,
            font_scale_aspect: 1f32,
            rotation: 0f32,
            color: theme.font_color,
        },
    );
}
