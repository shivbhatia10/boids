use crate::boid::Boid;
use macroquad::prelude::*;

pub struct Swarm {
    pub boids: Vec<Boid>,
    pub min_group_size: usize,
}

impl Swarm {
    pub fn new(num_boids: i32, min_group_size: usize) -> Self {
        let boids = (0..num_boids).map(|_| Boid::new_random()).collect();
        Swarm {
            boids,
            min_group_size,
        }
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

    fn group_by_position(&mut self, rows: i32, cols: i32) -> Vec<Vec<usize>> {
        let x_incr = screen_width() / cols as f32;
        let y_incr = screen_height() / rows as f32;
        let total_groups = (rows * cols) as usize;
        let mut groups: Vec<Vec<usize>> = vec![Vec::new(); total_groups];

        // Add random offset to break up grid alignment
        let x_offset = rand::gen_range(-x_incr * 0.3, x_incr * 0.3);
        let y_offset = rand::gen_range(-y_incr * 0.3, y_incr * 0.3);

        for (i, boid) in &mut self.boids.iter().enumerate() {
            let x_index = ((boid.position.x + x_offset) / x_incr).floor() as i32;
            let y_index = ((boid.position.y + y_offset) / y_incr).floor() as i32;

            // Ensure boid is always assigned to a valid group by clamping indices
            let x_index = x_index.clamp(0, cols - 1);
            let y_index = y_index.clamp(0, rows - 1);

            let group_index = (y_index * cols + x_index) as usize;

            if group_index < total_groups {
                groups[group_index].push(i);
            }
        }
        groups
    }

    pub fn _color_by_position(&mut self, rows: i32, cols: i32) {
        let groups = self.group_by_position(rows, cols);

        for (i, group) in groups.iter().enumerate() {
            let row = (i as i32) / cols;
            let col = (i as i32) % cols;

            for &boid_index in group {
                self.boids[boid_index].color = if (row + col) % 2 == 0 { RED } else { BLUE }
            }
        }
    }

    // Steer toward the average velocity of nearby boids
    pub fn alignment(&mut self, rows: i32, cols: i32, factor: f32) {
        let groups = self.group_by_position(rows, cols);

        for group in groups {
            if group.len() <= self.min_group_size {
                continue;
            }

            let mut avg_velocity = Vec2::ZERO;
            for &boid_index in &group {
                avg_velocity += self.boids[boid_index].velocity;
            }
            avg_velocity /= group.len() as f32;
            avg_velocity = avg_velocity.normalize();

            for &boid_index in &group {
                // Add a larger random perturbation to prevent perfect alignment
                let jitter = Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
                self.boids[boid_index].velocity = ((1.0 - factor)
                    * self.boids[boid_index].velocity
                    + factor * (avg_velocity + jitter))
                    .normalize();
            }
        }
    }

    // Steer toward the average position of nearby boids
    pub fn cohesion(&mut self, rows: i32, cols: i32, factor: f32) {
        let groups = self.group_by_position(rows, cols);

        for group in groups {
            if group.len() < self.min_group_size {
                continue;
            }

            let mut avg_position = Vec2::ZERO;
            for &boid_index in &group {
                avg_position += self.boids[boid_index].position;
            }
            avg_position /= group.len() as f32;

            for &boid_index in &group {
                // Adjust cohesion strength based on distance to center
                let distance = (avg_position - self.boids[boid_index].position).length();
                let distance_factor = (distance / 100.0).min(1.0); // Stronger pull when further away
                let adjusted_factor = factor * distance_factor;

                let direction = (avg_position - self.boids[boid_index].position).normalize();
                self.boids[boid_index].velocity = ((1.0 - adjusted_factor)
                    * self.boids[boid_index].velocity
                    + adjusted_factor * direction)
                    .normalize();
            }
        }
    }

    // Steer away from nearby boids to avoid crowding
    pub fn separation(&mut self, rows: i32, cols: i32, factor: f32) {
        let groups = self.group_by_position(rows, cols);
        let min_distance = 15.0; // Minimum distance to avoid division by zero
        let perception_radius = 35.0; // Only consider boids within this radius

        for group in groups {
            if group.len() <= self.min_group_size {
                continue;
            }

            for &boid_index in &group {
                let mut separation_force = Vec2::ZERO;
                let mut count = 0;

                for &other_index in &group {
                    if other_index == boid_index {
                        continue;
                    }

                    let other_position = self.boids[other_index].position;
                    let my_position = self.boids[boid_index].position;

                    let offset = my_position - other_position;
                    let distance = offset.length();

                    // Only respond to boids within perception radius
                    if distance < perception_radius {
                        // The closer the boid, the stronger the repulsion (1/distance)
                        // Also normalize the direction vector
                        let repulsion_strength = 15.0 / distance.max(min_distance);
                        separation_force += offset.normalize() * repulsion_strength;
                        count += 1;
                    }
                }

                // Only adjust velocity if we've detected nearby boids
                if count > 0 {
                    // Normalize the separation force
                    if separation_force.length_squared() > 0.0 {
                        separation_force = separation_force.normalize();
                    }

                    // Mix the current velocity with the separation force
                    self.boids[boid_index].velocity = ((1.0 - factor)
                        * self.boids[boid_index].velocity
                        + factor * separation_force)
                        .normalize();
                }
            }
        }
    }

    // Steer away from screen edges
    pub fn edge_avoidance(&mut self, margin: f32, factor: f32) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        for boid in &mut self.boids {
            let mut avoidance_force = Vec2::ZERO;

            // Left edge
            if boid.position.x < margin {
                avoidance_force.x += (margin - boid.position.x) / margin;
            }
            // Right edge
            if boid.position.x > screen_w - margin {
                avoidance_force.x -= (boid.position.x - (screen_w - margin)) / margin;
            }
            // Top edge
            if boid.position.y < margin {
                avoidance_force.y += (margin - boid.position.y) / margin;
            }
            // Bottom edge
            if boid.position.y > screen_h - margin {
                avoidance_force.y -= (boid.position.y - (screen_h - margin)) / margin;
            }

            // Apply the avoidance force if it exists
            if avoidance_force.length_squared() > 0.0 {
                avoidance_force = avoidance_force.normalize();
                boid.velocity =
                    ((1.0 - factor) * boid.velocity + factor * avoidance_force).normalize();
            }
        }
    }
}
