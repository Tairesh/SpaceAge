use crate::assets::Assets;
use crate::colors::Colors;
use crate::savefile::{delete, load, savefiles, savefiles_exists};
use crate::scenes::{bg, easy_back, GameScene, Scene, Transition};
use crate::ui::{Button, Horizontal, Label, Position, Positionate, UiSprite, Vertical};
use crate::VERSION;
use chrono::{DateTime, Local};
use geometry::Vec2;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

pub struct LoadWorld {
    sprites: Vec<Rc<RefCell<dyn UiSprite>>>,
}

impl LoadWorld {
    pub fn new(assets: &Assets, ctx: &mut Context) -> Self {
        let savefiles = savefiles();
        let mut sprites: Vec<Rc<RefCell<dyn UiSprite>>> =
            Vec::with_capacity(savefiles.len() * 5 + 2);
        sprites.push(bg(assets));
        // TODO: make a shortcut for scene title
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Load world:",
            assets.fonts.handel32.clone(),
            Colors::ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -200.0 },
            },
        ))));
        let height = savefiles.len() as f32 * 50.0;
        // TODO: Add scroll if there are too many savefiles
        let mut y = f32::max(-height / 2.0, -155.0);
        const KEYS: [Key; 10] = [
            Key::Num1,
            Key::Num2,
            Key::Num3,
            Key::Num4,
            Key::Num5,
            Key::Num6,
            Key::Num7,
            Key::Num8,
            Key::Num9,
            Key::Num0,
        ];
        for (i, savefile) in savefiles.iter().enumerate() {
            sprites.push(Rc::new(RefCell::new(Button::empty(
                // TODO: add some hint for this shortkeys
                if i <= 10 {
                    vec![(KEYS[i], None)]
                } else {
                    vec![]
                },
                Vec2::new(500.0, 40.0),
                Position {
                    x: Horizontal::AtWindowCenterByCenter { offset: -20.0 },
                    y: Vertical::AtWindowCenterByCenter { offset: y },
                },
                Transition::CustomEvent(format!("load:{}", savefile.path.to_str().unwrap())),
            ))));
            sprites.push(Rc::new(RefCell::new(Button::text(
                if i <= 10 {
                    vec![(KEYS[i], Some(KeyModifier::Alt))]
                } else {
                    vec![]
                },
                "Delete",
                assets.fonts.consolab18.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: 240.0 },
                    y: Vertical::AtWindowCenterByCenter { offset: y },
                },
                Transition::CustomEvent(format!("del:{}", savefile.path.to_str().unwrap())),
            ))));
            let name = Rc::new(RefCell::new(Label::new(
                format!("{} ({})", savefile.galaxy_name(), savefile.character_name()),
                assets.fonts.handel24.clone(),
                Colors::LIGHT_YELLOW,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -265.0 },
                    y: Vertical::AtWindowCenterByCenter { offset: y - 4.0 },
                },
            )));
            let name_size = name.borrow_mut().calc_size(ctx);
            sprites.push(name);
            sprites.push(Rc::new(RefCell::new(Label::new(
                &savefile.version,
                assets.fonts.consolab12.clone(),
                if savefile.version.as_str() == VERSION {
                    Colors::LIME
                } else {
                    Colors::RED
                },
                Position {
                    x: Horizontal::AtWindowCenterByLeft {
                        offset: -255.0 + name_size.x,
                    },
                    y: Vertical::AtWindowCenterByBottom { offset: y - 4.0 },
                },
            ))));
            let time: DateTime<Local> = savefile.time.into();
            sprites.push(Rc::new(RefCell::new(Label::new(
                time.format("%Y.%m.%d %H:%M:%S").to_string().as_str(),
                assets.fonts.consolab12.clone(),
                Colors::LIGHT_YELLOW,
                Position {
                    x: Horizontal::AtWindowCenterByLeft {
                        offset: -255.0 + name_size.x,
                    },
                    y: Vertical::AtWindowCenterByTop { offset: y },
                },
            ))));
            y += 50.0;
        }
        Self { sprites }
    }
}

impl Scene for LoadWorld {
    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn UiSprite>>>> {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: &str) -> Option<Transition> {
        let mut parts = event.split(':');
        match (parts.next(), parts.next()) {
            (Some("load"), Some(path)) => load(path.as_ref()).map(|s| {
                if s.has_character() {
                    Transition::LoadWorld(s)
                } else {
                    Transition::Replace(GameScene::CreateCharacter(s))
                }
            }),
            (Some("del"), Some(path)) => {
                let path = path.parse::<PathBuf>().unwrap();
                delete(&path);
                Some(if savefiles_exists() {
                    Transition::Replace(GameScene::LoadWorld)
                } else {
                    Transition::Pop
                })
            }
            (_, _) => unreachable!(),
        }
    }
}
