use macroquad::prelude::*;

pub struct Theme {
    pub background_color: Color,
    pub font_color: Color,
}

pub const DARK_THEME: Theme = Theme {
    background_color: Color {
        r: 0.0627f32,
        g: 0.0627f32,
        b: 0.0627f32,
        a: 1f32,
    },
    font_color: Color {
        r: 1f32,
        g: 1f32,
        b: 0.902f32,
        a: 1f32,
    },
};

pub const LIGHT_THEME: Theme = Theme {
    background_color: Color {
        r: 1f32,
        g: 1f32,
        b: 0.902f32,
        a: 1f32,
    },
    font_color: Color {
        r: 0.0627f32,
        g: 0.0627f32,
        b: 0.0627f32,
        a: 1f32,
    },
};
