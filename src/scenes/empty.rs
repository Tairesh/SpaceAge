use crate::colors::Colors;
use crate::scenes::{easy_back, Scene, Transition};
use tetra::{graphics, Context, Event};

pub struct Empty {}

impl Scene for Empty {
    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::SPACE_VIOLET);
    }
}
