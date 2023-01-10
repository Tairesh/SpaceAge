use crate::assets::Assets;
use crate::colors::Colors;
use crate::scenes::{easy_back, Scene, Transition};
use crate::sprites::bg::Bg;
use crate::sprites::meshy::HoverableMesh;
use crate::sprites::position::{Horizontal, Position};
use crate::sprites::sprite::Sprite;
use geometry::Vec2;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::{window, Context, Event};

pub struct Terminal {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl Terminal {
    pub fn new(ctx: &mut Context, _assets: &Assets) -> Self {
        let bg = Rc::new(RefCell::new(Bg::new(Colors::SPACE_VIOLET)));
        let window_height = window::get_height(ctx) as f32;
        let mfd_size = Vec2::new(window_height / 2.0, window_height / 2.0);
        let mfd = Rectangle::new(0.0, 0.0, mfd_size.x, mfd_size.y);
        let left_screen = Rc::new(RefCell::new(HoverableMesh::new(
            Mesh::rectangle(ctx, ShapeStyle::Fill, mfd).unwrap(),
            Colors::GRAY.with_alpha(0.5),
            Colors::DARK_ORANGE_RED.with_alpha(0.5),
            mfd_size,
            Position::vertical_center(0.0, Horizontal::AtWindowCenterByRight { offset: -50.0 }),
        )));
        let right_screen = Rc::new(RefCell::new(HoverableMesh::new(
            Mesh::rectangle(ctx, ShapeStyle::Fill, mfd).unwrap(),
            Colors::GRAY.with_alpha(0.5),
            Colors::DARK_ORANGE_RED.with_alpha(0.5),
            mfd_size,
            Position::vertical_center(0.0, Horizontal::AtWindowCenterByLeft { offset: 50.0 }),
        )));
        Self {
            sprites: vec![bg, left_screen, right_screen],
        }
    }
}

impl Scene for Terminal {
    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
