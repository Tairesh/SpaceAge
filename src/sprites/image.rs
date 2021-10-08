#![allow(dead_code)]
use crate::sprites::position::Position;
use crate::sprites::sprite::{Colorize, Draw, Positionate, Sprite, Update};
use crate::{Rect, Vec2};
use tetra::graphics::{Color, DrawParams, NineSlice, Rectangle, Texture};
use tetra::Context;

pub struct Image {
    texture: Texture,
    region: Option<Rectangle>,
    color: Option<Color>,
    nine_slice: Option<(NineSlice, f32, f32)>,
    scale: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Image {
    pub fn new(texture: Texture, position: Position) -> Self {
        Image {
            texture,
            region: None,
            color: None,
            nine_slice: None,
            scale: Vec2::new(1.0, 1.0),
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_nineslice(mut self, nineslice: NineSlice, width: f32, height: f32) -> Self {
        self.nine_slice = Some((nineslice, width, height));
        self
    }
}

impl Draw for Image {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let params = DrawParams::new()
            .position(Vec2::new(rect.x, rect.y))
            .scale(self.scale)
            .color(self.color.unwrap_or(Color::WHITE));
        if let Some((nine_slice, width, height)) = &self.nine_slice {
            self.texture
                .draw_nine_slice(ctx, nine_slice, *width, *height, params);
        } else if let Some(region) = self.region {
            self.texture.draw_region(ctx, region, params);
        } else {
            self.texture.draw(ctx, params);
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Image {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        let size = if let Some(region) = self.region {
            (region.width, region.height)
        } else {
            let (w, h) = self.texture.size();
            (w as f32, h as f32)
        };
        Vec2::new(size.0 * self.scale.x, size.1 * self.scale.y)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for Image {
    fn color(&self) -> Color {
        self.color.unwrap_or(Color::WHITE)
    }

    fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
}

impl Update for Image {}
impl Sprite for Image {}
