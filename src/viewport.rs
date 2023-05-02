use super::*;

pub struct Viewport {
    position: (f32, f32),
    size: (f32, f32),

    window_size: (i32, i32),
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            size: (1.0, 1.0),

            window_size: (1280, 720),
        }
    }

    pub fn from_values(position: (f32, f32), size: (f32, f32), window_size: (i32, i32)) -> Self {
        Self {
            position,
            size,

            window_size,
        }
    }

    pub fn set(&self, draw: &mut Draw) {
        draw.set_size(
            self.window_size.0 as f32 * self.size.0,
            self.window_size.1 as f32 * self.size.1,
        )
    }

    pub fn position(&mut self) -> &mut (f32, f32) {
        &mut self.position
    }
    pub fn size(&mut self) -> &mut (f32, f32) {
        &mut self.size
    }

    pub fn window_size(&mut self) -> &mut (i32, i32) {
        &mut self.window_size
    }
}