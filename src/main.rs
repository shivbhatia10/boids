mod boid;
mod swarm;

use macroquad::prelude::*;
use swarm::Swarm;

#[macroquad::main("Boids")]
async fn main() {
    let mut swarm = Swarm::new(1_000);
    let (alignment_rows, alignment_cols) = (10, 15);
    let (cohesion_rows, cohesion_cols) = (3, 5);
    let (separation_rows, separation_cols) = (20, 30);
    loop {
        clear_background(BLACK);

        // swarm.color_by_position(rows, cols);
        swarm.alignment(alignment_rows, alignment_cols, 0.1);
        swarm.cohesion(cohesion_rows, cohesion_cols, 0.001);
        swarm.separation(separation_rows, separation_cols, 0.01);

        swarm.update();
        swarm.draw();

        next_frame().await
    }
}
