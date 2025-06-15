mod boid;
mod swarm;

use macroquad::prelude::*;
use swarm::Swarm;

#[macroquad::main("Boids")]
async fn main() {
    let mut swarm = Swarm::new(5_000, 5);
    let (alignment_rows, alignment_cols) = (10, 10);
    let (cohesion_rows, cohesion_cols) = (3, 5);
    let (separation_rows, separation_cols) = (30, 30);

    let mut frame_count = 0;
    let mut total_update_time = 0.0;

    loop {
        clear_background(BLACK);

        // Time the behavior updates and position updates
        let update_start = get_time();

        // swarm.color_by_position(rows, cols);
        swarm.alignment(alignment_rows, alignment_cols, 0.02);
        swarm.cohesion(cohesion_rows, cohesion_cols, 0.01);
        swarm.separation(separation_rows, separation_cols, 0.06);
        swarm.edge_avoidance(20.0, 0.1); // Add edge avoidance

        swarm.update();

        let update_time = (get_time() - update_start) as f32 * 1000.0;
        total_update_time += update_time;
        frame_count += 1;

        swarm.draw();

        let num_alive = swarm.boids.iter().filter(|b| b.is_alive()).count();

        // Print performance stats every 60 frames
        if frame_count % 60 == 0 {
            let avg_update_time = total_update_time / 60.0;
            debug!(
                "Alive: {}, Avg update time: {:.2}ms, FPS: {:.1}",
                num_alive,
                avg_update_time,
                get_fps()
            );
            total_update_time = 0.0;
        }

        next_frame().await
    }
}
