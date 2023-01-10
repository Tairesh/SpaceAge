use crate::assets::Assets;
use crate::colors::Colors;
use crate::savefile::savefiles_exists;
use crate::scenes::{bg, GameScene, Scene, Transition};
use crate::ui::{Button, Disable, Horizontal, Label, Position, UiSprite, Vertical};
use crate::{TITLE, VERSION};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::Context;

pub struct MainMenu {
    sprites: Vec<Rc<RefCell<dyn UiSprite>>>,
    select_btn: Rc<RefCell<Button>>,
}

impl MainMenu {
    pub fn new(assets: &Assets) -> Self {
        let logo = Rc::new(RefCell::new(Label::new(
            TITLE,
            assets.fonts.logo.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -200.0 },
            },
        )));
        let version = Rc::new(RefCell::new(Label::new(
            VERSION,
            assets.fonts.consolab18.clone(),
            Colors::DARK_ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -178.0 },
            },
        )));
        let select_btn = Rc::new(RefCell::new(
            Button::text(
                vec![(Key::E, None)],
                "[e] Select world",
                assets.fonts.consolab18.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                    y: Vertical::AtWindowCenterByCenter { offset: 0.0 },
                },
                Transition::Push(GameScene::LoadWorld),
            )
            .with_disabled(true),
        ));
        let create_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::C, None)],
            "[c] Create new world",
            assets.fonts.consolab18.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 50.0 },
            },
            Transition::Push(GameScene::CreateWorld),
        )));
        let settings_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::S, None)],
            "[s] Settings",
            assets.fonts.consolab18.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 100.0 },
            },
            Transition::Push(GameScene::Settings),
        )));
        let exit_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::X, None)],
            "[x] Exit",
            assets.fonts.consolab18.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 150.0 },
            },
            Transition::Quit,
        )));
        Self {
            sprites: vec![
                bg(assets),
                logo,
                version,
                select_btn.clone(),
                create_btn,
                settings_btn,
                exit_btn,
            ],
            select_btn,
        }
    }
}

impl Scene for MainMenu {
    fn on_open(&mut self, _ctx: &mut Context) {
        self.select_btn
            .borrow_mut()
            .set_disabled(!savefiles_exists());
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn UiSprite>>>> {
        Some(&self.sprites)
    }
}
