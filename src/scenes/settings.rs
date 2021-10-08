use crate::assets::Assets;
use crate::colors::Colors;
use crate::scenes::{easy_back, Scene, Transition};
use crate::settings::{Settings, WindowMode};
use crate::sprites::button::Button;
use crate::sprites::image::Image;
use crate::sprites::label::Label;
use crate::sprites::position::{Horizontal, Position, Vertical};
use crate::sprites::sprite::{Positionate, Press, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

pub struct SettingsScene {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    radio_buttons: Vec<Rc<RefCell<Button>>>,
}

impl SettingsScene {
    pub fn new(assets: &Assets, settings: &Settings, ctx: &mut Context) -> Self {
        let bg = Rc::new(RefCell::new(Image::new(
            assets.images.bg.clone(),
            Position::center(),
        )));
        let title = Rc::new(RefCell::new(Label::new(
            "Settings",
            assets.fonts.astrolab.clone(),
            Colors::DARK_ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -200.0 },
            },
        )));
        let fullscreen_btn = Rc::new(RefCell::new(Button::fixed(
            "window_mode:fullscreen",
            vec![(Key::F, Some(KeyModifier::Alt))],
            "[Alt+F] Fullscreen",
            matches!(settings.window_mode(), WindowMode::Fullscreen),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 110.0 },
                y: Vertical::AtWindowCenterByTop { offset: -100.0 },
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("fullscreen".to_string()),
        )));
        let window_btn = Rc::new(RefCell::new(Button::fixed(
            "window_mode:window",
            vec![(Key::W, Some(KeyModifier::Alt))],
            "[Alt+W] Window",
            matches!(settings.window_mode(), WindowMode::Window),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 100.0 },
                y: Vertical::AtWindowCenterByTop { offset: -100.0 },
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("window".to_string()),
        )));
        let window_size = window_btn.borrow_mut().calc_size(ctx);
        let window_mode = Rc::new(RefCell::new(Label::new(
            "Window mode:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_size.x,
                },
                y: Vertical::AtWindowCenterByCenter {
                    offset: -102.0 + window_size.y / 2.0,
                },
            },
        )));
        let back_btn = Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowBottomByBottom { offset: -200.0 },
            },
            assets.fonts.consolab.clone(),
            Transition::Pop,
        )));

        SettingsScene {
            radio_buttons: vec![window_btn.clone(), fullscreen_btn.clone()],
            sprites: vec![bg, title, window_mode, window_btn, fullscreen_btn, back_btn],
        }
    }
}

impl Scene for SettingsScene {
    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: &str) -> Option<Transition> {
        for sprite in self.radio_buttons.iter() {
            if !sprite.borrow().id().ends_with(event) {
                sprite.borrow_mut().unpress();
            }
        }
        match event {
            "window" => {
                println!("window");
                Some(Transition::ChangeWindowMode(WindowMode::Window))
            }
            "fullscreen" => Some(Transition::ChangeWindowMode(WindowMode::Fullscreen)),
            _ => unreachable!(),
        }
    }
}
