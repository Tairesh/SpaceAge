#![allow(dead_code)]
use crate::scenes::Transition;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Draw, Positionate, Sprite, Update};
use crate::{Rect, Vec2};
use tetra::graphics::{DrawParams, NineSlice, Texture};
use tetra::input::{Key, MouseButton};
use tetra::{input, Context};

pub struct Alert {
    texture: Texture,
    nineslice: NineSlice,
    scale: Vec2,
    width: f32,
    height: f32,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Alert {
    pub fn new(
        width: f32,
        height: f32,
        texture: Texture,
        nineslice: NineSlice,
        position: Position,
    ) -> Self {
        Alert {
            texture,
            nineslice,
            scale: Vec2::new(3.0, 3.0),
            width,
            height,
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

impl Draw for Alert {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.texture.draw_nine_slice(
            ctx,
            &self.nineslice,
            self.width / self.scale.x,
            self.height / self.scale.y,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .scale(self.scale),
        )
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Alert {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Alert {
    fn update(&mut self, ctx: &mut Context, focused: bool, blocked: &[Rect]) -> Option<Transition> {
        if focused {
            return None;
        }
        if input::is_key_pressed(ctx, Key::Escape) {
            return Some(Transition::Pop);
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let mouse = input::get_mouse_position(ctx);
            if !self.rect.unwrap().contains_point(mouse) {
                if blocked.iter().any(|r| r.contains_point(mouse)) {
                    return None;
                }
                return Some(Transition::Pop);
            }
        }
        None
    }
}
impl Sprite for Alert {}
