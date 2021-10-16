use crate::assets::Assets;
use crate::scenes::{bg, easy_back, GameScene, Scene, Transition};
use crate::sprites::button::Button;
use crate::sprites::position::{Position, Vertical};
use crate::sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::{Context, Event};

pub struct GameMenu {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl GameMenu {
    pub fn new(assets: &Assets) -> Self {
        let back = Rc::new(RefCell::new(Button::new(
            vec![(Key::R, None)],
            "[r] Resume",
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: -50.0 }),
            assets.fonts.consolab18.clone(),
            Transition::Pop,
        )));
        let settings = Rc::new(RefCell::new(Button::new(
            vec![(Key::S, None)],
            "[S] Settings",
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 0.0 }),
            assets.fonts.consolab18.clone(),
            Transition::Replace(GameScene::Settings),
        )));
        let quit = Rc::new(RefCell::new(Button::new(
            vec![(Key::Q, None)],
            "[Q] Save and quit",
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 50.0 }),
            assets.fonts.consolab18.clone(),
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

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
