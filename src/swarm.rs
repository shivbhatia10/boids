use crate::boid::Boid;
use macroquad::prelude::*;

pub struct Swarm {
    pub boids: Vec<Boid>,
}

impl Swarm {
    pub fn new(num_boids: i32) -> Self {
        let boids = (0..num_boids).map(|_| Boid::new_random()).collect();
        Swarm { boids }
    }

    pub fn update(&mut self) {
        for boid in &mut self.boids {
            boid.update();
        }
    }

    pub fn draw(&self) {
        for boid in &self.boids {
            boid.draw();
        }
    }

    fn group_by_position(&mut self, x_incr: f32, y_incr: f32) -> Vec<Vec<usize>> {
        let rows = (screen_height() / y_incr).ceil() as i32;
        let cols = (screen_width() / x_incr).ceil() as i32;
        let total_groups = (rows * cols) as usize;
        let mut groups: Vec<Vec<usize>> = vec![Vec::new(); total_groups];

        for (i, boid) in &mut self.boids.iter().enumerate() {
            let x_index = (boid.position.x / x_incr).floor() as i32;
            let y_index = (boid.position.y / y_incr).floor() as i32;
            let group_index = (y_index * cols + x_index) as usize;

            if group_index < total_groups {
                groups[group_index].push(i);
            }
        }
        groups
    }

    pub fn color_by_position(&mut self, x_incr: f32, y_incr: f32) {
        let groups = self.group_by_position(x_incr, y_incr);
        for (i, group) in groups.iter().enumerate() {
            let color = Color::new(
                (i as f32 / groups.len() as f32).sin(),
                (i as f32 / groups.len() as f32).cos(),
                0.5,
                1.0,
            );
            for &boid_index in group {
                self.boids[boid_index].color = color;
            }
        }
    }
}
