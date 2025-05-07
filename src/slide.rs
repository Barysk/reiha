
/// For defining slide type
pub enum SlideType {
    Empty,
    Text(String),
    Image(String),
}

/// Slide struct
pub struct Slide {
    id: u32,
    r#type: SlideType,
    note: String
}

impl Slide {
    // FIXME: try to pass values here
    pub fn new() -> Self {
        Self {
            id: 0u32,
            r#type: SlideType::Empty,
        }
    }
    
    pub fn draw(&self) {

    }

}