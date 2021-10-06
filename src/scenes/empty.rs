use crate::colors::Colors;
use crate::scenes::{Scene, Transition};
use tetra::input::{Key, MouseButton};
use tetra::{graphics, Context, Event};

pub struct Empty {}

impl Scene for Empty {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> Transition {
        match event {
            Event::MouseButtonPressed {
                button: MouseButton::X1,
            }
            | Event::KeyPressed { key: Key::Escape } => Transition::Pop,
            _ => Transition::DoNothing,
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::SPACE_VIOLET);
    }
}
