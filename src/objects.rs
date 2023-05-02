use super::*;

pub trait Drawable {
    fn draw(&mut self, draw: &mut Draw, position: (f32, f32));
}

pub struct Wall {
    position: (f32, f32),
    size: (f32, f32),
    color: Color,
}

impl Wall {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            size: (32.0, 32.0),
            color: Color::WHITE,
        }
    }

    pub fn from_values(position: (f32, f32), size: (f32, f32), color: Color) -> Self {
        Self {
            position,
            size,
            color,
        }
    }

    pub fn position(&mut self) -> &mut (f32, f32) {
        &mut self.position
    }
    pub fn size(&mut self) -> &mut (f32, f32) {
        &mut self.size
    }
    pub fn color(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl Drawable for Wall {
    fn draw(&mut self, draw: &mut Draw, position: (f32, f32)) {
        self.position.0 += position.0;
        self.position.1 += position.1;

        draw.rect(self.position, self.size).fill_color(self.color);
    }
}