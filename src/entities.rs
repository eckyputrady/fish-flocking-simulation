use std::rc::Rc;

use macroquad::math::*;
use macroquad::prelude::*;

use crate::boid;
use crate::boid::Boid;
use crate::movement::{Movement, Transform2D};
use crate::render::{Animation, AnimationDefinition, RenderableTexture, TextureAtlas};

#[derive(Clone)]
pub struct Entity {
    pub transform: Option<Transform2D>,
    pub movement: Option<Movement>,
    pub config: Option<boid::Config>,
    pub animation: Option<Animation>,
    pub renderable_texture: Option<RenderableTexture>
}

pub fn query_mut<'a, T: TryFrom<&'a mut Entity>>(entities: &'a mut Vec<Entity>) -> Vec<T> {
    entities.iter_mut()
        .map(|e| T::try_from(e))
        .flatten()
        .collect()
}

pub fn query<'a, T: TryFrom<&'a Entity>>(entities: &'a mut Vec<Entity>) -> Vec<T> {
    entities.iter()
        .map(|e| T::try_from(e))
        .flatten()
        .collect()
}

impl <'a> TryFrom<&'a mut Entity> for Boid<'a> {
    type Error = ();

    fn try_from(value: &'a mut Entity) -> Result<Self, Self::Error> {
        match (&value.transform, &mut value.movement, &value.config) {
            (Some(transform), Some(movement), Some(config)) => Ok(Boid {
                pos: &transform.pos,
                vel: &mut movement.vel,
                config
            }),
            _ => Err(())
        }
    }
}

impl <'a> TryFrom<&'a mut Entity> for (&'a mut RenderableTexture, &'a mut Animation) {
    type Error = ();

    fn try_from(value: &'a mut Entity) -> Result<Self, Self::Error> {
        match (&mut value.renderable_texture, &mut value.animation) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(())
        }
    }
}

impl <'a> TryFrom<&'a Entity> for (&'a Transform2D, &'a RenderableTexture) {
    type Error = ();

    fn try_from(value: &'a Entity) -> Result<Self, Self::Error> {
        match (&value.transform, &value.renderable_texture) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(())
        }
    }
}

impl <'a> TryFrom<&'a mut Entity> for (&'a mut Transform2D, &'a Movement) {
    type Error = ();

    fn try_from(value: &'a mut Entity) -> Result<Self, Self::Error> {
        match (&mut value.transform, &value.movement) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(())
        }
    }
}

impl <'a> TryFrom<&'a mut Entity> for (&'a mut Transform2D, &'a mut Animation, &'a Movement) {
    type Error = ();

    fn try_from(value: &'a mut Entity) -> Result<Self, Self::Error> {
        match (&mut value.transform, &mut value.animation, &value.movement) {
            (Some(a), Some(b), Some(c)) => Ok((a, b, c)),
            _ => Err(())
        }
    }
}

pub async fn setup_entities<'a>() -> Vec<Entity> {
    let window_width = screen_width();
    let window_height = screen_height();
    let bounds_margin = 40.0;
    let bounds = Rect::new(bounds_margin, bounds_margin, window_width - bounds_margin * 2.0, window_height - bounds_margin * 2.0);
    let boid_config = boid::Config {
        bounds,
        neighbor_distance: 100.0,
        separation_distance: 20.0,
        separation_rule_weight: 0.3,
        avoidance_rule_weight: 0.06,
        cohesion_rule_weight: 0.01,
        alignment_rule_weight: 0.04,
        bounds_rule_weight: 0.02,
        exploration_rule_weight: 0.05,
        field_of_view: -0.9,
        max_speed: 80.0,
        flock_id: 1,
        flock_to_avoid: Default::default()
    };

    let texture_atlas_fish = Rc::new(TextureAtlas {
        texture: load_texture("resources/fish_spritesheet.png").await.unwrap(),
        num_tiles: vec2(12.0, 8.0)
    });

    let fish_animation_definition_0 = Rc::new(AnimationDefinition {
        atlas: texture_atlas_fish.clone(),
        duration_per_frame: 0.2,
        frame_coords: vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(2.0, 0.0)]
    });

    let fish_animation_definition_1 = Rc::new(AnimationDefinition {
        frame_coords: vec![vec2(6.0, 4.0), vec2(7.0, 4.0), vec2(8.0, 4.0)],
        atlas: texture_atlas_fish.clone(),
        ..*fish_animation_definition_0
    });

    let fish_animation_definition_2 = Rc::new(AnimationDefinition {
        frame_coords: vec![vec2(6.0, 0.0), vec2(7.0, 0.0), vec2(8.0, 0.0)],
        atlas: texture_atlas_fish.clone(),
        ..*fish_animation_definition_0
    });

    let fish_animation_definition_3 = Rc::new(AnimationDefinition {
        frame_coords: vec![vec2(9.0, 0.0), vec2(10.0, 0.0), vec2(11.0, 0.0)],
        atlas: texture_atlas_fish.clone(),
        ..*fish_animation_definition_0
    });

    let texture_atlas_shark = Rc::new(TextureAtlas {
        texture: load_texture("resources/shark_spritesheet.png").await.unwrap(),
        num_tiles: vec2(3.0, 4.0)
    });

    let shark_animation_definition_0 = Rc::new(AnimationDefinition {
        atlas: texture_atlas_shark.clone(),
        duration_per_frame: 0.2,
        frame_coords: vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(2.0, 0.0)]
    });

    let shark_flock_id = 10;

    let shark_archetype_0 = Entity {
        transform: Some(Transform2D {
            pos: vec2(0.0, 0.0),
            rot_radians: 0.0
        }),
        movement: Some(Movement {
            vel: vec2(0.0, 0.0)
        }),
        config: Some(boid::Config {
            bounds,
            flock_id: shark_flock_id,
            neighbor_distance: 200.0,
            separation_distance: 100.0,
            separation_rule_weight: 0.0,
            avoidance_rule_weight: 0.0,
            cohesion_rule_weight: 0.0,
            alignment_rule_weight: 0.000,
            bounds_rule_weight: 0.002,
            exploration_rule_weight: 0.00,
            field_of_view: -0.9,
            max_speed: 40.0,
            flock_to_avoid: Default::default()
        }),
        animation: Some(Animation {
            definition: shark_animation_definition_0.clone(),
            tick: 0.0,
            speed: 1.0,
            frame_number: 0
        }),
        renderable_texture: Some(RenderableTexture {
            texture: texture_atlas_shark.texture,
            pos_offset: vec2(-40.0, -50.0),
            rot_offset_radians: -(6.28319 / 4.0),
            color: WHITE,
            params: DrawTextureParams {
                dest_size: Some(vec2(80.0, 80.0)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        })
    };

    let fish_archetype_0 = Entity {
        transform: Some(Transform2D {
            pos: vec2(0.0, 0.0),
            rot_radians: 0.0
        }),
        movement: Some(Movement {
            vel: vec2(0.0, 0.0)
        }),
        config: Some(boid::Config {
            flock_to_avoid: vec![shark_flock_id].into_iter().collect(),
            ..boid_config.clone()
        }),
        animation: Some(Animation {
            definition: fish_animation_definition_0.clone(),
            tick: 0.0,
            speed: 1.0,
            frame_number: 0
        }),
        renderable_texture: Some(RenderableTexture {
            texture: texture_atlas_fish.texture,
            pos_offset: vec2(-20.0, -28.0),
            rot_offset_radians: -(6.28319 / 4.0),
            color: WHITE,
            params: DrawTextureParams {
                dest_size: Some(vec2(40.0, 40.0)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        })
    };

    let fish_archetype_1 = Entity {
        transform: Some(Transform2D {
            pos: vec2(0.0, 0.0),
            rot_radians: 0.0
        }),
        movement: Some(Movement {
            vel: vec2(0.0, 0.0)
        }),
        config: Some(boid::Config {
            flock_id: 1,
            flock_to_avoid: vec![shark_flock_id].into_iter().collect(),
            ..boid_config.clone()
        }),
        animation: Some(Animation {
            definition: fish_animation_definition_1.clone(),
            tick: 0.0,
            speed: 1.0,
            frame_number: 0
        }),
        renderable_texture: Some(RenderableTexture {
            texture: texture_atlas_fish.texture,
            pos_offset: vec2(-20.0, -28.0),
            rot_offset_radians: -(6.28319 / 4.0),
            color: WHITE,
            params: DrawTextureParams {
                dest_size: Some(vec2(40.0, 40.0)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        })
    };

    let fish_archetype_2 = Entity {
        transform: Some(Transform2D {
            pos: vec2(0.0, 0.0),
            rot_radians: 0.0
        }),
        movement: Some(Movement {
            vel: vec2(0.0, 0.0)
        }),
        config: Some(boid::Config {
            flock_id: 2,
            flock_to_avoid: vec![shark_flock_id].into_iter().collect(),
            ..boid_config.clone()
        }),
        animation: Some(Animation {
            definition: fish_animation_definition_2.clone(),
            tick: 0.0,
            speed: 1.0,
            frame_number: 0
        }),
        renderable_texture: Some(RenderableTexture {
            texture: texture_atlas_fish.texture,
            pos_offset: vec2(-20.0, -28.0),
            rot_offset_radians: -(6.28319 / 4.0),
            color: WHITE,
            params: DrawTextureParams {
                dest_size: Some(vec2(40.0, 40.0)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        })
    };

    let fish_archetype_3 = Entity {
        transform: Some(Transform2D {
            pos: vec2(0.0, 0.0),
            rot_radians: 0.0
        }),
        movement: Some(Movement {
            vel: vec2(0.0, 0.0)
        }),
        config: Some(boid::Config {
            flock_id: 3,
            flock_to_avoid: vec![shark_flock_id].into_iter().collect(),
            ..boid_config.clone()
        }),
        animation: Some(Animation {
            definition: fish_animation_definition_3.clone(),
            tick: 0.0,
            speed: 1.0,
            frame_number: 0
        }),
        renderable_texture: Some(RenderableTexture {
            texture: texture_atlas_fish.texture,
            pos_offset: vec2(-20.0, -28.0),
            rot_offset_radians: -(6.28319 / 4.0),
            color: WHITE,
            params: DrawTextureParams {
                dest_size: Some(vec2(40.0, 40.0)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        })
    };

    let mut entities = vec![];
    for _ignored in 0..30 {
        entities.push(Entity {
            transform: Some(Transform2D {
                pos: vec2(rand::gen_range(0.0, window_width), rand::gen_range(0.0, window_height)),
                rot_radians: 0.0
            }),
            movement: Some(Movement {
                vel: vec2(rand::gen_range(-window_width, window_width), rand::gen_range(-window_height, window_height))
            }),
            ..fish_archetype_0.clone()
        });
        entities.push(Entity {
            transform: Some(Transform2D {
                pos: vec2(rand::gen_range(0.0, window_width), rand::gen_range(0.0, window_height)),
                rot_radians: 0.0
            }),
            movement: Some(Movement {
                vel: vec2(rand::gen_range(-window_width, window_width), rand::gen_range(-window_height, window_height))
            }),
            ..fish_archetype_1.clone()
        });
        entities.push(Entity {
            transform: Some(Transform2D {
                pos: vec2(rand::gen_range(0.0, window_width), rand::gen_range(0.0, window_height)),
                rot_radians: 0.0
            }),
            movement: Some(Movement {
                vel: vec2(rand::gen_range(-window_width, window_width), rand::gen_range(-window_height, window_height))
            }),
            ..fish_archetype_2.clone()
        });
        entities.push(Entity {
            transform: Some(Transform2D {
                pos: vec2(rand::gen_range(0.0, window_width), rand::gen_range(0.0, window_height)),
                rot_radians: 0.0
            }),
            movement: Some(Movement {
                vel: vec2(rand::gen_range(-window_width, window_width), rand::gen_range(-window_height, window_height))
            }),
            ..fish_archetype_3.clone()
        });
    }

    for _ignored in 0..3 {
        entities.push(Entity {
            transform: Some(Transform2D {
                pos: vec2(rand::gen_range(0.0, window_width), rand::gen_range(0.0, window_height)),
                rot_radians: 0.0
            }),
            movement: Some(Movement {
                vel: vec2(rand::gen_range(-window_width, window_width), rand::gen_range(-window_height, window_height))
            }),
            ..shark_archetype_0.clone()
        })
    }

    entities
}