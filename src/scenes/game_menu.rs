use crate::assets::Assets;
use crate::scenes::{bg, easy_back, GameScene, Scene, Transition};
use crate::ui::{Button, Position, UiSprite, Vertical};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::{Context, Event};

pub struct GameMenu {
    sprites: Vec<Rc<RefCell<dyn UiSprite>>>,
}

impl GameMenu {
    pub fn new(assets: &Assets) -> Self {
        let back = Rc::new(RefCell::new(Button::text(
            vec![(Key::R, None)],
            "[r] Back to game",
            assets.fonts.consolab18.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: -50.0 }),
            Transition::Pop,
        )));
        let settings = Rc::new(RefCell::new(Button::text(
            vec![(Key::S, None)],
            "[s] Settings",
            assets.fonts.consolab18.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 0.0 }),
            Transition::Replace(GameScene::Settings),
        )));
        let quit = Rc::new(RefCell::new(Button::text(
            vec![(Key::Q, None)],
            "[q] Save and quit",
            assets.fonts.consolab18.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 50.0 }),
            Transition::UnloadWorld,
        )));
        Self {
            sprites: vec![bg(assets), back, settings, quit],
        }
    }
}

impl Scene for GameMenu {
    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn UiSprite>>>> {
        Some(&self.sprites)
    }
}
