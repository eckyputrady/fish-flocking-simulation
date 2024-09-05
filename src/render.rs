use std::rc::Rc;
use macroquad::prelude::*;
use crate::movement::Transform2D;

#[derive(Clone)]
pub struct TextureAtlas {
    pub texture: Texture2D,
    pub num_tiles: Vec2,
}

impl TextureAtlas {
    fn get_texture_rect(&self, coord: Vec2) -> Rect {
        let tile_width = self.texture.width() / self.num_tiles.x;
        let tile_height = self.texture.height() / self.num_tiles.y;
        Rect::new(coord.x * tile_width, coord.y * tile_height, tile_width, tile_height)
    }
}

#[derive(Clone)]
pub struct AnimationDefinition {
    pub atlas: Rc<TextureAtlas>,
    pub duration_per_frame: f32,
    pub frame_coords: Vec<Vec2>
}

#[derive(Clone)]
pub struct Animation {
    pub definition: Rc<AnimationDefinition>,
    pub tick: f32,
    pub frame_number: usize,
    pub speed: f32,
}

impl<'a> Animation {
    fn tick(&mut self, tick: f32) {
        self.tick += tick * self.speed;
        if self.tick > self.definition.duration_per_frame {
            self.tick -= self.definition.duration_per_frame;
            self.frame_number = (self.frame_number + 1) % self.definition.frame_coords.len()
        }
    }
}

#[derive(Clone)]
pub struct RenderableTexture {
    pub texture: Texture2D,
    pub pos_offset: Vec2,
    pub rot_offset_radians: f32,
    pub color: Color,
    pub params: DrawTextureParams
}

pub fn animation_system(input: &mut Vec<(&mut RenderableTexture, &mut Animation)>, tick: f32) {
    for (renderable_texture, animation) in input {
        animation.tick(tick);
        let frame_coord = animation.definition.frame_coords.get(animation.frame_number).unwrap();
        let source = animation.definition.atlas.get_texture_rect(*frame_coord);
        renderable_texture.params.source = Some(source);
    }
}

pub fn renderable_texture_system(input: &Vec<(&Transform2D, &RenderableTexture)>) {
    for (transorm, renderable_texture) in input {
        let draw_pos = transorm.pos + renderable_texture.pos_offset;
        draw_texture_ex(
            &renderable_texture.texture,
            draw_pos.x,
            draw_pos.y,
            renderable_texture.color,
            DrawTextureParams {
                rotation: transorm.rot_radians + renderable_texture.rot_offset_radians,
                pivot: Some(transorm.pos),
                ..renderable_texture.params
            }
        );
    }
}