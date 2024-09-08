use std::collections::HashSet;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Config {
    pub bounds_margin: f32,
    pub neighbor_distance: f32,
    pub separation_distance: f32,
    pub avoidance_distance: f32,
    pub separation_rule_weight: f32,
    pub cohesion_rule_weight: f32,
    pub alignment_rule_weight: f32,
    pub bounds_rule_weight: f32,
    pub exploration_rule_weight: f32,
    pub avoidance_rule_weight: f32,
    pub field_of_view: f32,
    pub max_speed: f32,
    pub flock_id: u8,
    pub flock_to_avoid: HashSet<u8>
}

pub struct Boid<'a> {
    pub pos: &'a Vec2,
    pub vel: &'a mut Vec2,
    pub config: &'a Config
}


pub fn boids_system(boids: &mut Vec<Boid>, bounds: &Rect) {
    for i in 0..boids.len() {
        let cur = boids.get(i).unwrap();
        let neighbor_indices = neighbor_indices(cur, boids, cur.config.neighbor_distance, cur.config.field_of_view);
        let neighbors = select(&boids, &neighbor_indices);
        let my_flock: Vec<&Boid> = neighbors.iter().filter(|b| b.config.flock_id == cur.config.flock_id).map(|a| *a).collect();
        let to_avoids:Vec<&Boid> = neighbors.iter().filter(|b| cur.config.flock_to_avoid.contains(&b.config.flock_id)).map(|a| *a).collect();
        let acceleration = separation_rule(cur, &neighbors, cur.config.separation_distance, cur.config.separation_rule_weight)
            + cohesion_rule(cur, &my_flock, cur.config.separation_distance, cur.config.cohesion_rule_weight)
            + alignment_rule(&my_flock, cur.config.alignment_rule_weight)
            + bounds_rule(cur, bounds, cur.config.bounds_margin, cur.config.max_speed, cur.config.bounds_rule_weight)
            + exploration_rule(cur, cur.config.max_speed, cur.config.exploration_rule_weight)
            + avoidance_rule(cur, &to_avoids, cur.config.avoidance_distance, cur.config.avoidance_rule_weight)
            ;

        let cur = boids.get_mut(i).unwrap();
        *cur.vel += acceleration;
        *cur.vel = limit_vel(*cur.vel, cur.config.max_speed);
    }
}

fn exploration_rule(cur: &Boid, speed: f32, weight: f32) -> Vec2 {
    let random_vel = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
    if random_vel.dot(*cur.vel) > 0.2 {
        random_vel * speed * weight
    } else {
        vec2(0.0, 0.0)
    }
}

fn limit_vel(vel: Vec2, speed: f32) -> Vec2 {
    let actual_speed = vel.length();
    if actual_speed > speed {
        vel / actual_speed * speed
    } else {
        vel
    }
}

fn alignment_rule(boids: &Vec<&Boid>, weight: f32) -> Vec2 {
    weight * 
        boids.iter()
            .map(|b| (1.0, *b.vel))
            .reduce(|(a1, ad), (b1, bd)| (a1 + b1, ad + bd))
            .map(|(n, d)| d / n)
            .unwrap_or(vec2(0.0, 0.0))
}

fn cohesion_rule(cur: &Boid, boids: &Vec<&Boid>, min_distance: f32, weight: f32) -> Vec2 {
    weight * 
        boids.iter()
            .filter(|b| (*b.pos - *cur.pos).length_squared() > min_distance)
            .map(|b| (1.0, *b.pos))
            .reduce(|(a1, ad), (b1, bd)| (a1 + b1, ad + bd))
            .map(|(n, d)| d / n)
            .map(|center| center - *cur.pos)
            .unwrap_or(vec2(0.0, 0.0))
}

fn separation_rule(cur: &Boid, boids: &Vec<&Boid>, max_distance: f32, weight: f32) -> Vec2 {
    let max_distance_squared = max_distance * max_distance;
    weight * 
        boids.iter()
            .map(|b| (1.0, *cur.pos - *b.pos))
            .filter(|(_ignored, d)| d.length_squared() <= max_distance_squared)
            .reduce(|(a1, ad), (b1, bd)| (a1 + b1, ad + bd))
            .map(|(n, d)| d / n)
            .unwrap_or(vec2(0.0, 0.0))
}

fn avoidance_rule(cur: &Boid, boids: &Vec<&Boid>, max_distance: f32, weight: f32) -> Vec2 {
    let max_distance_squared = max_distance * max_distance;
    weight * 
        boids.iter()
            .map(|b| (1.0, *cur.pos - *b.pos))
            .filter(|(_ignored, d)| d.length_squared() <= max_distance_squared)
            .reduce(|(a1, ad), (b1, bd)| (a1 + b1, ad + bd))
            .map(|(n, d)| d / n)
            .unwrap_or(vec2(0.0, 0.0))
}

fn bounds_rule(boid: &Boid, bounds: &Rect, margin: f32, speed: f32, weight: f32) -> Vec2 {
    let rect = Rect::new(bounds.x + margin, bounds.y + margin, bounds.w - 2.0 * margin, bounds.h - 2.0 * margin);
    let x = if boid.pos.x < rect.left() {
        speed
    } else if boid.pos.x > rect.right() {
        -speed
    } else {
        0.0
    };

    let y = if boid.pos.y < rect.top() {
        speed
    } else if boid.pos.y > rect.bottom() {
        -speed
    } else {
        0.0
    };

    vec2(x, y) * weight
}

fn select<'a, T>(list: &'a Vec<T>, indices: &'a Vec<usize>) -> Vec<&'a T> {
    let mut vec: Vec<&T> = vec![];
    for index in indices {
        if let Some(m) = list.get(*index) {
            vec.push(m)
        }
    }

    vec
}

fn neighbor_indices(cur: &Boid, boids: &Vec<Boid>, max_distance: f32, field_of_view: f32) -> Vec<usize> {
    let max_distance_squared = max_distance * max_distance;
    boids.iter().enumerate()
        .filter(|(_idx, other)| is_neighbor(cur, other, max_distance_squared, field_of_view))
        .map(|(idx, _other)| idx)
        .collect()
}

fn is_neighbor(cur: &Boid, other: &Boid, max_distance_squared: f32, field_of_view: f32) -> bool {
    if std::ptr::eq(cur, other) {
        return false;
    }

    if cur.pos.distance_squared(*other.pos) > max_distance_squared {
        return false;
    }

    let to_other = *other.pos - *cur.pos;
    let is_visible_for_me = cur.vel.normalize().dot(to_other.normalize()) > field_of_view;
    if !is_visible_for_me {
        return false;
    }

    return true;
}