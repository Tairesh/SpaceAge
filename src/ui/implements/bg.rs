use crate::ui::{Draw, Focus, Position, Positionate, UiSprite, Update};
use geometry::{Rect, Vec2};
use tetra::graphics::Color;
use tetra::{graphics, Context};

pub struct Bg {
    color: Color,
    visible: bool,
}

impl Bg {
    pub fn new(color: Color) -> Self {
        Self {
            color,
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
        Position::center()
    }

    fn set_position(&mut self, _position: Position) {
        // do nothing
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        Vec2::default()
    }

    fn rect(&self) -> Rect {
        Rect::default()
    }

    fn set_rect(&mut self, _rect: Rect) {
        // do nothing
    }
}

impl Update for Bg {}

impl Focus for Bg {}

impl UiSprite for Bg {}
