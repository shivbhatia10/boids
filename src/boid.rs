use macroquad::prelude::*;

#[derive(Debug)]
pub struct Boid {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Color,
}

impl Boid {
    pub fn new_random() -> Self {
        let position = Vec3::new(
            rand::gen_range(-50.0, 50.0),
            rand::gen_range(-50.0, 50.0),
            rand::gen_range(-50.0, 50.0),
        );
        let velocity = Vec3::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        )
        .normalize();
        Boid {
            position,
            velocity,
            color: Color::new((velocity.x + 1.0) * 0.5, (velocity.y + 1.0) * 0.5, 1.0, 1.0),
        }
    }

    pub fn update(&mut self) {
        // Prevent zero velocity
        if !self.is_alive() {
            self.velocity = Vec3::new(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
            )
            .normalize();
        }

        self.position += self.velocity;

        // Clamp position to 3D bounds
        let bound = 50.0;
        self.position.x = self.position.x.clamp(-bound, bound);
        self.position.y = self.position.y.clamp(-bound, bound);
        self.position.z = self.position.z.clamp(-bound, 0.0);
    }

    pub fn draw(&self) {
        // Draw a small cube to represent the boid
        let size = 1.0;
        draw_cube(self.position, vec3(size, size, size), None, self.color);

        // Draw a line showing velocity direction
        let velocity_end = self.position + self.velocity.normalize() * 3.0;
        draw_line_3d(self.position, velocity_end, WHITE);
    }

    pub fn is_alive(&self) -> bool {
        self.velocity.length() > 0.001
    }
}
