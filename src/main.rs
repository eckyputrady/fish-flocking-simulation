mod boid;
mod entities;
mod render;
mod movement;
mod polish;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(miniquad::date::now() as _);
    let mut entities = entities::setup_entities().await;
    loop {
        let elapsed = get_frame_time();
        boid::boids_system(&mut entities::query_mut(&mut entities));
        movement::movement_system(&mut entities::query_mut(&mut entities), elapsed);
        polish::fish_polish_system(&mut entities::query_mut(&mut entities));
        render::animation_system(&mut entities::query_mut(&mut entities), elapsed);
        clear_background(DARKBLUE);
        render::renderable_texture_system(&entities::query(&mut entities));
        next_frame().await
    }
}