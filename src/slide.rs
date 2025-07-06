use macroquad::prelude::*;

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
