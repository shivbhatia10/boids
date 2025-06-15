use macroquad::prelude::*;

#[derive(Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
}

impl Boid {
    pub fn new_random() -> Self {
        let position = Vec2::new(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
        );
        let velocity =
            Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize();
        Boid {
            position,
            velocity,
            color: Color::new(velocity.x, velocity.y, 1.0, 1.0),
        }
    }

    pub fn update(&mut self) {
        // Prevent zero velocity
        if !self.is_alive() {
            self.velocity =
                Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize();
        }

        self.position += self.velocity;

        // Clamp position to screen bounds instead of wrapping
        self.position.x = self.position.x.clamp(0.0, screen_width());
        self.position.y = self.position.y.clamp(0.0, screen_height());
    }

    pub fn draw(&self) {
        // draw_ellipse(self.position.x, self.position.y, 20.0, 20.0, 0.0, BLUE);
        draw_triangle(
            self.position + self.velocity.normalize().rotate(Vec2::Y) * 5.0,
            self.position - self.velocity.normalize().rotate(Vec2::Y) * 5.0,
            self.position + self.velocity.normalize() * 10.0,
            self.color,
        );
    }

    pub fn is_alive(&self) -> bool {
        self.velocity.length() > 0.001
    }
}
