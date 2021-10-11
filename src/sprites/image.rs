#![allow(dead_code)]
use crate::sprites::position::Position;
use crate::sprites::sprite::{Colorize, Draw, Positionate, Sprite, Update};
use crate::{Rect, Vec2};
use tetra::graphics::{Color, DrawParams, NineSlice, Rectangle, Texture};
use tetra::{window, Context};

pub struct Image {
    texture: Texture,
    region: Option<Rectangle>,
    color: Option<Color>,
    nine_slice: Option<(NineSlice, f32, f32)>,
    scale: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    repeat: bool,
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
            repeat: false,
        }
    }

    pub fn repeat(texture: Texture) -> Self {
        Self {
            texture,
            region: None,
            color: None,
            nine_slice: None,
            scale: Vec2::new(1.0, 1.0),
            position: Position::center(),
            rect: None,
            visible: true,
            repeat: true,
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
        } else if self.repeat {
            let (w, h) = window::get_size(ctx);
            let w_count = ((w as f32 / rect.w).ceil() / 2.0) as i32;
            let h_count = ((h as f32 / rect.h).ceil() / 2.0) as i32;
            for i in -w_count..=w_count {
                for j in -h_count..=h_count {
                    let pos = Vec2::new(rect.x + i as f32 * rect.w, rect.y + j as f32 * rect.h);
                    self.texture.draw(ctx, params.clone().position(pos));
                }
            }
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

    fn set_color<C: Into<Color>>(&mut self, value: C) {
        self.color = Some(value.into());
    }
}

impl Update for Image {}
impl Sprite for Image {}
