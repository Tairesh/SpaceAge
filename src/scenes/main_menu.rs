use crate::assets::Assets;
use crate::colors::Colors;
use crate::savefile::savefiles_exists;
use crate::scenes::{GameScene, Scene, Transition};
use crate::sprites::button::Button;
use crate::sprites::image::Image;
use crate::sprites::label::Label;
use crate::sprites::position::{Horizontal, Position, Vertical};
use crate::sprites::sprite::{Disable, Sprite};
use crate::{TITLE, VERSION};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::Context;

pub struct MainMenu {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    select_btn: Rc<RefCell<Button>>,
}

impl MainMenu {
    pub fn new(assets: &Assets) -> Self {
        let bg = Rc::new(RefCell::new(Image::new(
            assets.images.bg.clone(),
            Position::center(),
        )));
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
                y: Vertical::AtWindowCenterByBottom { offset: -180.0 },
            },
        )));
        let select_btn = Rc::new(RefCell::new(
            Button::new(
                vec![(Key::E, None)],
                "[e] Select world",
                Position {
                    x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                    y: Vertical::AtWindowCenterByCenter { offset: 0.0 },
                },
                assets.fonts.consolab18.clone(),
                Transition::Push(GameScene::LoadWorld),
            )
            .with_disabled(true),
        ));
        let create_btn = Rc::new(RefCell::new(Button::new(
            vec![(Key::C, None)],
            "[c] Create new world",
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 50.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::Push(GameScene::CreateWorld),
        )));
        let settings_btn = Rc::new(RefCell::new(Button::new(
            vec![(Key::S, None)],
            "[s] Settings",
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 100.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::Push(GameScene::Settings),
        )));
        let exit_btn = Rc::new(RefCell::new(Button::new(
            vec![(Key::X, None)],
            "[x] Exit",
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 150.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::Quit,
        )));
        Self {
            sprites: vec![
                bg,
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

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
