use crate::assets::Assets;
use crate::colors::Colors;
use crate::scenes::{bg, easy_back, Scene, Transition};
use crate::settings::{Settings, WindowMode};
use crate::sprites::button::Button;
use crate::sprites::label::Label;
use crate::sprites::position::{Horizontal, Position, Vertical};
use crate::sprites::sprite::{Positionate, Press, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::window::WindowPosition;
use tetra::{window, Context, Event};

pub struct SettingsScene {
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    window: Rc<RefCell<Button>>,
    fullscreen: Rc<RefCell<Button>>,
}

impl SettingsScene {
    pub fn new(assets: &Assets, settings: Rc<RefCell<Settings>>, ctx: &mut Context) -> Self {
        let title = Rc::new(RefCell::new(Label::new(
            "Settings",
            assets.fonts.handel32.clone(),
            Colors::ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -200.0 },
            },
        )));
        let fullscreen_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::F, Some(KeyModifier::Alt))],
            "[Alt+F] Fullscreen",
            matches!(settings.borrow().window_mode(), WindowMode::Fullscreen),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 110.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -100.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("fullscreen".to_string()),
        )));
        let window_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::W, Some(KeyModifier::Alt))],
            "[Alt+W] Window",
            matches!(settings.borrow().window_mode(), WindowMode::Window),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 100.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -100.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("window".to_string()),
        )));
        let window_size = window_btn.borrow_mut().calc_size(ctx);
        let window_mode = Rc::new(RefCell::new(Label::new(
            "Window mode:",
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_size.x,
                },
                y: Vertical::AtWindowCenterByCenter { offset: -106.0 },
            },
        )));
        let back_btn = Rc::new(RefCell::new(Button::new(
            vec![(Key::Escape, None)],
            "[Esc] Back",
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowBottomByBottom { offset: -200.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::Pop,
        )));

        SettingsScene {
            settings,
            window: window_btn.clone(),
            fullscreen: fullscreen_btn.clone(),
            sprites: vec![
                bg(assets),
                title,
                window_mode,
                window_btn,
                fullscreen_btn,
                back_btn,
            ],
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

    fn custom_event(&mut self, ctx: &mut Context, event: &str) -> Option<Transition> {
        match event {
            "window" => {
                self.fullscreen.borrow_mut().unpress();
                let mut settings = self.settings.borrow_mut();
                settings.fullscreen = false;
                if window::is_fullscreen(ctx) {
                    window::set_fullscreen(ctx, false).ok();
                }
                window::set_decorated(ctx, true);
                window::set_size(ctx, settings.width as i32, settings.height as i32).ok();
                window::set_position(
                    ctx,
                    WindowPosition::Centered(0),
                    WindowPosition::Centered(0),
                );
                None
            }
            "fullscreen" => {
                self.window.borrow_mut().unpress();
                self.settings.borrow_mut().fullscreen = true;
                window::set_fullscreen(ctx, true).ok();
                None
            }
            _ => unreachable!(),
        }
    }
}

impl Drop for SettingsScene {
    fn drop(&mut self) {
        self.settings.borrow_mut().save();
    }
}
