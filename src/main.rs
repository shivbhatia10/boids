mod boid;
mod swarm;

use macroquad::prelude::*;
use swarm::Swarm;

#[macroquad::main("MyGame")]
async fn main() {
    let mut swarm = Swarm::new(1_000);
    loop {
        clear_background(BLACK);
        swarm.color_by_position(10.0, 10.0);
        swarm.update();
        swarm.draw();

        next_frame().await
    }
}
