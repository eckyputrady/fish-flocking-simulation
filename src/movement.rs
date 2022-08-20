use macroquad::prelude::*;

#[derive(Clone)]
pub struct Transform2D {
    pub pos: Vec2,
    pub rot_radians: f32
}

#[derive(Clone)]
pub struct Movement {
    pub vel: Vec2
}

pub fn movement_system(input: &mut Vec<(&mut Transform2D, &Movement)>, elapsed: f32) {
    for (transform, movement) in input {
        transform.pos += movement.vel * elapsed
    }
}