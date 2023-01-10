use crate::assets::Assets;
use crate::colors::Colors;
use crate::scenes::{bg, easy_back, Scene, Transition};
use crate::settings::Settings;
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
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    window: Rc<RefCell<Button>>,
    fullscreen: Rc<RefCell<Button>>,
}

impl SettingsScene {
    pub fn new(assets: &Assets, ctx: &mut Context) -> Self {
        let settings = Settings::instance();
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
            assets.fonts.consolab18.clone(),
            settings.window.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 110.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -100.0 },
            },
            Transition::CustomEvent("fullscreen".to_string()),
        )));
        let window_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::W, Some(KeyModifier::Alt))],
            "[Alt+W] Window",
            assets.fonts.consolab18.clone(),
            !settings.window.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 100.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -100.0 },
            },
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
                y: Vertical::AtWindowCenterByCenter { offset: -105.0 },
            },
        )));
        let back_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.fonts.consolab18.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowBottomByBottom { offset: -200.0 },
            },
            Transition::Pop,
        )));

        SettingsScene {
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
                let mut settings = Settings::instance();
                settings.window.fullscreen = false;
                if window::is_fullscreen(ctx) {
                    window::set_fullscreen(ctx, false).ok();
                }
                window::set_decorated(ctx, true);
                window::set_size(ctx, settings.window.width, settings.window.height).ok();
                let current_monitor = window::get_current_monitor(ctx).unwrap_or(0);
                window::set_position(
                    ctx,
                    WindowPosition::Centered(current_monitor),
                    WindowPosition::Centered(current_monitor),
                );
                None
            }
            "fullscreen" => {
                self.window.borrow_mut().unpress();
                Settings::instance().window.fullscreen = true;
                if let Ok((width, height)) = window::get_current_monitor_size(ctx) {
                    window::set_size(ctx, width, height).ok();
                }
                window::set_fullscreen(ctx, true).ok();
                None
            }
            _ => unreachable!(),
        }
    }
}

impl Drop for SettingsScene {
    fn drop(&mut self) {
        Settings::instance().save();
    }
}
