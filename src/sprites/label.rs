#![allow(dead_code)]
use crate::sprites::position::Position;
use crate::sprites::sprite::{Colorize, Draw, Positionate, Sprite, Stringify, Update};
use crate::{Rect, Vec2};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams};
use tetra::Context;

pub struct Label {
    text: Text,
    color: Color,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Label {
    pub fn new<C: Into<String>>(text: C, font: Font, color: Color, position: Position) -> Self {
        Label {
            text: Text::new(text, font),
            color,
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn hidden<C: Into<String>>(text: C, font: Font, color: Color, position: Position) -> Self {
        Label {
            text: Text::new(text, font),
            color,
            position,
            rect: None,
            visible: false,
        }
    }
}

impl Draw for Label {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .color(self.color),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Label {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let rect = self.text.get_bounds(ctx).unwrap();
        Vec2::new(rect.width, rect.height)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for Label {
    fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Stringify for Label {
    fn value(&self) -> String {
        self.text.content().to_string()
    }

    fn set_value<C: Into<String>>(&mut self, value: C) {
        self.text.set_content(value);
    }
}

impl Update for Label {
    fn block_mouse(&self) -> bool {
        false
    }
}
impl Sprite for Label {}
