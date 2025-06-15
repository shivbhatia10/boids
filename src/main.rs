mod boid;
mod swarm;

use macroquad::prelude::*;
use swarm::Swarm;

#[macroquad::main("Boids")]
async fn main() {
    let mut swarm = Swarm::new(2_000, 5);
    let (alignment_rows, alignment_cols) = (10, 10);
    let (cohesion_rows, cohesion_cols) = (3, 5);
    let (separation_rows, separation_cols) = (30, 30);
    loop {
        clear_background(BLACK);

        // swarm.color_by_position(rows, cols);
        swarm.alignment(alignment_rows, alignment_cols, 0.1);
        swarm.cohesion(cohesion_rows, cohesion_cols, 0.01);
        swarm.separation(separation_rows, separation_cols, 0.06);
        swarm.edge_avoidance(10.0, 0.15); // Add edge avoidance

        swarm.update();
        swarm.draw();

        let num_alive = swarm.boids.iter().filter(|b| b.is_alive()).count();
        debug!("{}", num_alive);

        next_frame().await
    }
}
