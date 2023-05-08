use super::*;

pub mod wall;
pub mod player;

pub trait Drawable {
    fn draw(&mut self, draw: &mut Draw, position: (f32, f32));
}

pub struct Wall {
    position: (f32, f32),
    size: (f32, f32),

    color: Color,
}

pub struct Player {
    position: (f32, f32),
    size: (f32, f32),

    color: Color,

    speed: f32,
}