use super::*;

impl Player {
    pub fn new() -> Self {
        Self {
            position: (0.0, 64.0),
            size: (32.0, 32.0),

            color: Color::WHITE,

            speed: 40.0,
        }
    }

    pub fn from_values(position: (f32, f32), size: (f32, f32), color: Color, speed: f32) -> Self {
        Self {
            position,
            size,

            color,

            speed,
        }
    }

    pub fn move_pos(&mut self, delta: f32, direction: Direction) {
        if direction == Direction::UP {
            self.position.1 -= delta * self.speed;
        } else if direction == Direction::DOWN {
            self.position.1 += delta * self.speed;
        } else if direction == Direction::LEFT {
            self.position.0 -= delta * self.speed;
        } else if direction == Direction::RIGHT {
            self.position.0 += delta * self.speed;
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

    pub fn speed(&mut self) -> &mut f32 {
        &mut self.speed
    }
}

impl Drawable for Player {
    fn draw(&mut self, draw: &mut Draw, position: (f32, f32)) {
        self.position.0 += position.0;
        self.position.1 += position.1;

        draw.rect(self.position, self.size).fill_color(self.color);
    }
}