use crate::scenes::Transition;
use crate::sprites::position::Position;
use crate::{Rect, Vec2};
use tetra::graphics::Color;
use tetra::Context;

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context);
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

pub trait Positionate {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn calc_size(&mut self, ctx: &mut Context) -> Vec2;
    fn rect(&self) -> Rect;
    fn set_rect(&mut self, rect: Rect);
    fn calc_rect(&mut self, owner_size: Vec2, window_size: (i32, i32)) -> Rect {
        let left_top = self.position().as_vec(owner_size, window_size);
        Rect::new(left_top.x, left_top.y, owner_size.x, owner_size.y)
    }
    fn positionate(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        let size = self.calc_size(ctx);
        let rect = self.calc_rect(size, window_size);
        self.set_rect(rect);
    }
}

pub trait Update {
    // focused means there is some focused sprite on the scene
    // blocked is rects of sprites above current one
    // TODO: implement a way to tell there is an yes-or-no-style alert, blocking even hovering
    fn update(
        &mut self,
        _ctx: &mut Context,
        _focused: bool,
        _blocked: &[Rect],
    ) -> Option<Transition> {
        None
    }
}

pub trait Disable {
    fn disabled(&self) -> bool;
    fn set_disabled(&mut self, disabled: bool);
}

pub trait Colorize {
    fn color(&self) -> Color;
    fn set_color(&mut self, color: Color);
}

pub trait Stringify {
    fn value(&self) -> String;
    fn set_value<C: Into<String>>(&mut self, value: C);
}

pub trait Hover {
    fn on_hovered(&mut self);
    fn off_hovered(&mut self);
}

pub trait Press {
    fn on_pressed(&mut self);
    fn off_pressed(&mut self);
    fn unpress(&mut self);
}

pub trait Sprite: Draw + Positionate + Update {
    fn block_mouse(&self) -> bool {
        self.visible()
    }
    fn focused(&self) -> bool {
        false
    }
    fn set_focused(&mut self, _focused: bool) {}
}
