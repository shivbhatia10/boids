use crate::boid::Boid;
use macroquad::prelude::*;

pub struct Swarm {
    pub boids: Vec<Boid>,
    pub min_group_size: usize,
    // Cache for performance optimizations
    screen_width: f32,
    screen_height: f32,
    // Pre-allocated random offsets to avoid generating them every frame
    group_offset_x: f32,
    group_offset_y: f32,
    frame_counter: u32,
}

impl Swarm {
    pub fn new(num_boids: i32, min_group_size: usize) -> Self {
        let boids = (0..num_boids).map(|_| Boid::new_random()).collect();
        let screen_width = screen_width();
        let screen_height = screen_height();
        Swarm {
            boids,
            min_group_size,
            screen_width,
            screen_height,
            group_offset_x: 0.0,
            group_offset_y: 0.0,
            frame_counter: 0,
        }
    }

    pub fn update(&mut self) {
        // Update cached screen dimensions less frequently
        self.frame_counter += 1;
        if self.frame_counter % 60 == 0 {
            self.screen_width = screen_width();
            self.screen_height = screen_height();
            // Update random offsets less frequently to reduce random number generation
            self.group_offset_x = rand::gen_range(-1.0, 1.0);
            self.group_offset_y = rand::gen_range(-1.0, 1.0);
        }

        for boid in &mut self.boids {
            boid.update(self.screen_width, self.screen_height);
        }
    }

    // Consolidated method to update all behaviors efficiently
    pub fn update_behaviors(
        &mut self,
        alignment_params: (i32, i32, f32),
        cohesion_params: (i32, i32, f32),
        separation_params: (i32, i32, f32),
        edge_params: (f32, f32),
    ) {
        // Update alignment
        self.alignment(alignment_params.0, alignment_params.1, alignment_params.2);
        // Update cohesion
        self.cohesion(cohesion_params.0, cohesion_params.1, cohesion_params.2);
        // Update separation
        self.separation(
            separation_params.0,
            separation_params.1,
            separation_params.2,
        );
        // Update edge avoidance
        self.edge_avoidance(edge_params.0, edge_params.1);
    }

    pub fn draw(&self) {
        for boid in &self.boids {
            boid.draw();
        }
    }

    fn group_by_position(&self, rows: i32, cols: i32) -> Vec<Vec<usize>> {
        let x_incr = self.screen_width / cols as f32;
        let y_incr = self.screen_height / rows as f32;
        let total_groups = (rows * cols) as usize;
        let mut groups = vec![Vec::new(); total_groups];

        // Use cached offsets that change less frequently
        let x_offset = self.group_offset_x * x_incr * 0.3;
        let y_offset = self.group_offset_y * y_incr * 0.3;

        for (i, boid) in self.boids.iter().enumerate() {
            let x_index = ((boid.position.x + x_offset) / x_incr).floor() as i32;
            let y_index = ((boid.position.y + y_offset) / y_incr).floor() as i32;

            // Ensure boid is always assigned to a valid group by clamping indices
            let x_index = x_index.clamp(0, cols - 1);
            let y_index = y_index.clamp(0, rows - 1);

            let group_index = (y_index * cols + x_index) as usize;
            groups[group_index].push(i);
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

        // Pre-calculate jitter values to reduce random number generation
        let jitter_x = self.group_offset_x * 0.5;
        let jitter_y = self.group_offset_y * 0.5;
        let jitter = Vec2::new(jitter_x, jitter_y);

        for group in &groups {
            if group.len() <= self.min_group_size {
                continue;
            }

            let mut avg_velocity = Vec2::ZERO;
            for &boid_index in group {
                avg_velocity += self.boids[boid_index].velocity;
            }
            avg_velocity /= group.len() as f32;

            // Only normalize if length is significant
            if avg_velocity.length_squared() > 0.001 {
                avg_velocity = avg_velocity.normalize();
            }

            for &boid_index in group {
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

        for group in &groups {
            if group.len() < self.min_group_size {
                continue;
            }

            let mut avg_position = Vec2::ZERO;
            for &boid_index in group {
                avg_position += self.boids[boid_index].position;
            }
            avg_position /= group.len() as f32;

            for &boid_index in group {
                let offset = avg_position - self.boids[boid_index].position;
                // Use length_squared for distance comparison to avoid sqrt
                let distance_squared = offset.length_squared();
                if distance_squared > 0.001 {
                    let distance = distance_squared.sqrt();
                    let distance_factor = (distance / 100.0).min(1.0);
                    let adjusted_factor = factor * distance_factor;

                    let direction = offset / distance; // Normalize by dividing by length
                    self.boids[boid_index].velocity = ((1.0 - adjusted_factor)
                        * self.boids[boid_index].velocity
                        + adjusted_factor * direction)
                        .normalize();
                }
            }
        }
    }

    // Steer away from nearby boids to avoid crowding
    pub fn separation(&mut self, rows: i32, cols: i32, factor: f32) {
        let groups = self.group_by_position(rows, cols);
        let perception_radius_squared = 35.0 * 35.0; // Square the perception radius

        for group in &groups {
            if group.len() <= self.min_group_size {
                continue;
            }

            for &boid_index in group {
                let mut separation_force = Vec2::ZERO;
                let mut count = 0;
                let my_position = self.boids[boid_index].position;

                for &other_index in group {
                    if other_index == boid_index {
                        continue;
                    }

                    let offset = my_position - self.boids[other_index].position;
                    let distance_squared = offset.length_squared();

                    // Only respond to boids within perception radius (using squared distance)
                    if distance_squared < perception_radius_squared && distance_squared > 0.001 {
                        let distance = distance_squared.sqrt();
                        // The closer the boid, the stronger the repulsion (1/distance)
                        let repulsion_strength = 15.0 / distance.max(15.0);
                        separation_force += (offset / distance) * repulsion_strength; // Normalize by dividing by distance
                        count += 1;
                    }
                }

                // Only adjust velocity if we've detected nearby boids
                if count > 0 && separation_force.length_squared() > 0.0 {
                    separation_force = separation_force.normalize();

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
        // Use cached screen dimensions
        let screen_w = self.screen_width;
        let screen_h = self.screen_height;

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
