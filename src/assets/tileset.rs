use crate::ascii::cp437::char_to_point;
use tetra::{
    graphics::{DrawParams, Rectangle, Texture},
    Context, Result,
};

pub struct TileSet {
    pub texture: Texture,
}

impl TileSet {
    pub const TILE_SIZE: (i32, i32) = (12, 12);

    pub fn new(ctx: &mut Context) -> Result<Self> {
        Ok(Self {
            texture: Texture::from_encoded(ctx, include_bytes!("../../inc/img/12x12.png"))?,
        })
    }

    pub fn draw<P: Into<DrawParams>>(&self, ctx: &mut Context, ch: char, params: P) {
        self.texture
            .draw_region(ctx, TileSet::char_to_rectangle(ch), params);
    }

    fn char_to_rectangle(ch: char) -> Rectangle {
        let point = char_to_point(ch);
        Rectangle::new(
            point.x as f32 * TileSet::TILE_SIZE.0 as f32,
            point.y as f32 * TileSet::TILE_SIZE.1 as f32,
            TileSet::TILE_SIZE.0 as f32,
            TileSet::TILE_SIZE.1 as f32,
        )
    }
}
