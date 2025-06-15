mod boid;
mod swarm;

use macroquad::prelude::*;
use swarm::Swarm;

#[macroquad::main("Boids 3D")]
async fn main() {
    let mut swarm = Swarm::new(2_000, 5);
    let (alignment_rows, alignment_cols, alignment_depth) = (8, 8, 8);
    let (cohesion_rows, cohesion_cols, cohesion_depth) = (4, 4, 4);
    let (separation_rows, separation_cols, separation_depth) = (20, 20, 20);

    let mut frame_count = 0;
    let mut total_update_time = 0.0;

    loop {
        clear_background(BLACK);

        // Set up 3D camera
        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, 100.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        // Time the behavior updates and position updates
        let update_start = get_time();

        // swarm.color_by_position(rows, cols, depth);
        swarm.alignment(alignment_rows, alignment_cols, alignment_depth, 0.02);
        swarm.cohesion(cohesion_rows, cohesion_cols, cohesion_depth, 0.01);
        swarm.separation(separation_rows, separation_cols, separation_depth, 0.06);
        swarm.edge_avoidance(10.0, 0.1); // Add edge avoidance

        swarm.update();

        let update_time = (get_time() - update_start) as f32 * 1000.0;
        total_update_time += update_time;
        frame_count += 1;

        swarm.draw();

        // Switch back to 2D camera for UI
        set_default_camera();

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
