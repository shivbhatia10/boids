# boids

A boid simulation written in Rust with [macroquad](https://macroquad.rs/).

Uses three rules:

- Alignment: Steer towards the average heading of local flockmates.
- Cohesion: Steer towards the average position of local flockmates.
- Separation: Steer to avoid crowding local flockmates.
