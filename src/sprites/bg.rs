use crate::sprites::position::Position;
use crate::sprites::sprite::{Draw, Positionate, Sprite, Update};
use geometry::{Rect, Vec2};
use tetra::graphics::Color;
use tetra::{graphics, Context};

pub struct Bg {
    color: Color,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Bg {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::center(),
            rect: None,
            visible: true,
        }
    }
}

impl Draw for Bg {
    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, self.color);
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Bg {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        Vec2::default()
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Bg {}
impl Sprite for Bg {}
