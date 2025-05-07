use crate::slide::Slide;

pub struct Presentation {
    pub slides: Vec<Slide>,
    pub current_index: usize,
}

impl Presentation {
    pub fn next_slide(&mut self) {
        if self.current_index + 1 < self.slides.len() {
            self.current_index += 1;
        }
    }

    pub fn previous_slide(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn current_slide(&self) -> Option<&Slide> {
        self.slides.get(self.current_index)
    }
}