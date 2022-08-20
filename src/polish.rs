use crate::movement::{Movement, Transform2D};
use crate::render::Animation;

pub fn fish_polish_system(input: &mut Vec<(&mut Transform2D, &mut Animation, &Movement)>) {
    for (transform, animation, movement) in input {
        let speed = movement.vel.length();

        // facing the velocity vector direction (if they move fast enough)
        if speed > 25.0{
            transform.rot_radians = movement.vel.y.atan2(movement.vel.x);
        }

        // animation speed depends on the velocity
        animation.speed = speed / 100.0;
    }
}