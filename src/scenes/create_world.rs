use crate::assets::Assets;
use crate::colors::Colors;
use crate::savefile::{CreateFileError, SaveFile};
use crate::scenes::{easy_back, Scene, Transition};
use crate::sprites::button::Button;
use crate::sprites::image::Image;
use crate::sprites::input::TextInput;
use crate::sprites::label::Label;
use crate::sprites::position::{AnchorY, Horizontal, Position, Vertical};
use crate::sprites::sprite::{Draw, Positionate, Sprite, Stringify};
use rand::RngCore;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{graphics, Context, Event};

// TODO: random world name
fn random_seed() -> String {
    rand::thread_rng().next_u32().to_string()
}

pub struct CreateWorld {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    name_error: Rc<RefCell<Label>>,
    seed_input: Rc<RefCell<TextInput>>,
    seed_error: Rc<RefCell<Label>>,
}

impl CreateWorld {
    pub fn new(assets: &Assets, ctx: &mut Context) -> Self {
        // TODO: add galaxy type, size and preview
        let bg = Rc::new(RefCell::new(Image::new(
            assets.images.bg.clone(),
            Position::center(),
        )));
        let title = Rc::new(RefCell::new(Label::new(
            "Create new world:",
            assets.fonts.astrolab.clone(),
            Colors::DARK_ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -200.0 },
            },
        )));
        let name_label = Rc::new(RefCell::new(Label::new(
            "World name:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::AtWindowCenter { offset: -102.0 },
            },
        )));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            "Tadek",
            250.0,
            assets.fonts.nasa.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: -100.0 },
            },
        )));
        let name_error = Rc::new(RefCell::new(Label::hidden(
            "Savefile with this name already exists",
            assets.fonts.consolab.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 125.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -131.0 },
            },
        )));
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "World name shall not be empty!",
            assets.fonts.consolab.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 125.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -131.0 },
            },
        )));
        let seed_label = Rc::new(RefCell::new(Label::new(
            "World seed:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::AtWindowCenter { offset: -32.0 },
            },
        )));
        let seed_input = Rc::new(RefCell::new(TextInput::new(
            random_seed().as_str(),
            250.0,
            assets.fonts.nasa.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: -30.0 },
            },
        )));
        let seed_error = Rc::new(RefCell::new(Label::hidden(
            "Seed shall not be empty!",
            assets.fonts.consolab.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 125.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -61.0 },
            },
        )));
        let randomize_btn = Rc::new(RefCell::new(Button::new(
            "randomize",
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: AnchorY::Center.to_position(500.0),
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("randomize".to_string()),
        )));
        let randomize_size = randomize_btn.borrow_mut().calc_size(ctx);
        let back_btn = Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0 - 10.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
            assets.fonts.consolab.clone(),
            Transition::Pop,
        )));
        let create_btn = Rc::new(RefCell::new(Button::new(
            "create",
            vec![(Key::Enter, Some(KeyModifier::Alt))],
            "[Alt+Enter] Create",
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0 + 10.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("create".to_string()),
        )));
        CreateWorld {
            name_input: name_input.clone(),
            name_error: name_error.clone(),
            name_empty: name_empty.clone(),
            seed_input: seed_input.clone(),
            seed_error: seed_error.clone(),
            sprites: vec![
                bg,
                title,
                name_label,
                name_input,
                seed_label,
                seed_input,
                back_btn,
                randomize_btn,
                create_btn,
                name_error,
                seed_error,
                name_empty,
            ],
        }
    }
}

impl Scene for CreateWorld {
    fn update(&mut self, _ctx: &mut Context) -> Transition {
        let name = self.name_input.borrow();
        let mut name_empty = self.name_empty.borrow_mut();
        let mut name_error = self.name_error.borrow_mut();
        let seed = self.seed_input.borrow();
        let mut seed_error = self.seed_error.borrow_mut();
        if !name.danger() && name_empty.visible() {
            name_empty.set_visible(false);
        }
        if !name.danger() && name_error.visible() {
            name_error.set_visible(false);
        }
        if !seed.danger() && seed_error.visible() {
            seed_error.set_visible(false);
        }
        Transition::DoNothing
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> Transition {
        easy_back(event).unwrap_or(Transition::DoNothing)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::SPACE_VIOLET);
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: &str) -> Option<Transition> {
        match event {
            "randomize" => {
                // TODO: randomize world name
                self.seed_input
                    .borrow_mut()
                    .set_value(random_seed().as_str());
                None
            }
            "create" => {
                let seed = self.seed_input.borrow().value();
                let name = self.name_input.borrow().value();
                if seed.is_empty() {
                    self.seed_input.borrow_mut().set_danger(true);
                    self.seed_error.borrow_mut().set_visible(true);
                }
                if name.is_empty() {
                    self.name_input.borrow_mut().set_danger(true);
                    self.name_empty.borrow_mut().set_visible(true);
                    None
                } else {
                    let mut file = SaveFile::new(name.as_str(), seed.as_str());
                    match file.create() {
                        Ok(_) => Some(Transition::Pop),
                        Err(err) => {
                            match err {
                                CreateFileError::SystemError(err) => {
                                    panic!("Can't create savefile: {}", err)
                                }
                                CreateFileError::FileExists => {
                                    self.name_input.borrow_mut().set_danger(true);
                                    self.name_error.borrow_mut().set_visible(true);
                                }
                            }
                            None
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
