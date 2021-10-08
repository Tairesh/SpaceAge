use crate::assets::Assets;
use crate::astro::galaxy_class::GalaxyClass;
use crate::astro::galaxy_generator;
use crate::astro::galaxy_size::GalaxySize;
use crate::colors::Colors;
use crate::savefile::{CreateFileError, SaveFile};
use crate::scenes::{easy_back, Scene, Transition};
use crate::sprites::button::Button;
use crate::sprites::galaxy::Galaxy;
use crate::sprites::image::Image;
use crate::sprites::input::TextInput;
use crate::sprites::label::Label;
use crate::sprites::position::{Horizontal, Position, Vertical};
use crate::sprites::sprite::{Draw, Positionate, Sprite, Stringify};
use rand::distributions::Standard;
use rand::{Rng, RngCore};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{graphics, window, Context, Event};

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
    size_name: Rc<RefCell<Label>>,
    class_name: Rc<RefCell<Label>>,
    preview: Rc<RefCell<Galaxy>>,
    galaxy_size: GalaxySize,
    galaxy_class: GalaxyClass,
}

impl CreateWorld {
    pub fn new(assets: &Assets, ctx: &mut Context) -> Self {
        let right_column_width: f32 = 300.0;
        let bg = Rc::new(RefCell::new(Image::new(
            assets.images.bg.clone(),
            Position::center(),
        )));
        let title = Rc::new(RefCell::new(Label::new(
            "Create new world:",
            assets.fonts.astrolab.clone(),
            Colors::DARK_ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -200.0 },
            },
        )));
        let name_label = Rc::new(RefCell::new(Label::new(
            "Galaxy name:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -122.0 },
            },
        )));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            "Tadek",
            right_column_width,
            assets.fonts.nasa.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -120.0 },
            },
        )));
        let name_error = Rc::new(RefCell::new(Label::hidden(
            "Savefile with this name already exists",
            assets.fonts.consolab.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0,
                },
                y: Vertical::AtWindowCenterByBottom { offset: -151.0 },
            },
        )));
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "World name shall not be empty!",
            assets.fonts.consolab.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0,
                },
                y: Vertical::AtWindowCenterByBottom { offset: -151.0 },
            },
        )));
        let seed_label = Rc::new(RefCell::new(Label::new(
            "World seed:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -52.0 },
            },
        )));
        let seed = random_seed();
        let seed_input = Rc::new(RefCell::new(TextInput::new(
            seed.as_str(),
            right_column_width,
            assets.fonts.nasa.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -50.0 },
            },
        )));
        let seed_error = Rc::new(RefCell::new(Label::hidden(
            "Seed shall not be empty!",
            assets.fonts.consolab.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0,
                },
                y: Vertical::AtWindowCenterByBottom { offset: -81.0 },
            },
        )));
        let size_label = Rc::new(RefCell::new(Label::new(
            "Galaxy size:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 18.0 },
            },
        )));
        let size_left = Rc::new(RefCell::new(Button::new(
            "size_left",
            vec![],
            "<",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 20.0 },
            },
            assets.fonts.nasa.clone(),
            Transition::CustomEvent("size:left".to_string()),
        )));
        let galaxy_size = GalaxySize::Normal;
        let size_name = Rc::new(RefCell::new(Label::new(
            galaxy_size.name(),
            assets.fonts.nasa.clone(),
            Colors::DARK_ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 18.0 },
            },
        )));
        let size_right = Rc::new(RefCell::new(Button::new(
            "size_right",
            vec![],
            ">",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: right_column_width,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 20.0 },
            },
            assets.fonts.nasa.clone(),
            Transition::CustomEvent("size:right".to_string()),
        )));
        let class_label = Rc::new(RefCell::new(Label::new(
            "Galaxy class:",
            assets.fonts.nasa.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 88.0 },
            },
        )));
        let class_left = Rc::new(RefCell::new(Button::new(
            "class_left",
            vec![],
            "<",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 90.0 },
            },
            assets.fonts.nasa.clone(),
            Transition::CustomEvent("class:left".to_string()),
        )));
        let galaxy_class = GalaxyClass::Spiral;
        let class_name = Rc::new(RefCell::new(Label::new(
            galaxy_class.name(),
            assets.fonts.nasa.clone(),
            Colors::DARK_ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 88.0 },
            },
        )));
        let class_right = Rc::new(RefCell::new(Button::new(
            "class_right",
            vec![],
            ">",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: right_column_width,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 90.0 },
            },
            assets.fonts.nasa.clone(),
            Transition::CustomEvent("class:right".to_string()),
        )));
        let randomize_btn = Rc::new(RefCell::new(Button::new(
            "randomize",
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: Vertical::AtWindowCenterByTop { offset: 150.0 },
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("randomize".to_string()),
        )));
        let randomize_size = randomize_btn.borrow_mut().calc_size(ctx);
        let preview_btn = Rc::new(RefCell::new(Button::new(
            "preview",
            vec![(Key::P, None)],
            "[P] Preview",
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -5.0 },
                y: Vertical::AtWindowCenterByTop { offset: 150.0 },
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("preview".to_string()),
        )));
        let preview_size = preview_btn.borrow_mut().calc_size(ctx);
        let back_btn = Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -15.0 - preview_size.x,
                },
                y: Vertical::AtWindowCenterByTop { offset: 150.0 },
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
                    offset: randomize_size.x + 15.0,
                },
                y: Vertical::AtWindowCenterByTop { offset: 150.0 },
            },
            assets.fonts.consolab.clone(),
            Transition::CustomEvent("create".to_string()),
        )));
        let preview = Rc::new(RefCell::new(Galaxy::new(
            128,
            assets.fonts.astrolab.clone(),
            assets.fonts.nasa.clone(),
            Position::center(),
        )));
        CreateWorld {
            sprites: vec![
                bg,
                title,
                name_label,
                name_input.clone(),
                seed_label,
                seed_input.clone(),
                size_label,
                size_left,
                size_name.clone(),
                size_right,
                class_label,
                class_left,
                class_name.clone(),
                class_right,
                back_btn,
                preview_btn,
                randomize_btn,
                create_btn,
                name_error.clone(),
                seed_error.clone(),
                name_empty.clone(),
                preview.clone(),
            ],
            name_input,
            name_error,
            name_empty,
            seed_input,
            seed_error,
            size_name,
            class_name,
            preview,
            galaxy_size,
            galaxy_class,
        }
    }
}

impl Scene for CreateWorld {
    fn update(&mut self, _ctx: &mut Context, _focused: bool) -> Transition {
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

    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::SPACE_VIOLET);
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: &str) -> Option<Transition> {
        match event {
            "size:left" => {
                self.galaxy_size = self.galaxy_size.prev();
                self.size_name
                    .borrow_mut()
                    .set_value(self.galaxy_size.name());
                self.size_name
                    .borrow_mut()
                    .positionate(ctx, window::get_size(ctx));
                None
            }
            "size:right" => {
                self.galaxy_size = self.galaxy_size.next();
                self.size_name
                    .borrow_mut()
                    .set_value(self.galaxy_size.name());
                self.size_name
                    .borrow_mut()
                    .positionate(ctx, window::get_size(ctx));
                None
            }
            "class:left" => {
                self.galaxy_class = self.galaxy_class.prev();
                self.class_name
                    .borrow_mut()
                    .set_value(self.galaxy_class.name());
                self.class_name
                    .borrow_mut()
                    .positionate(ctx, window::get_size(ctx));
                None
            }
            "class:right" => {
                self.galaxy_class = self.galaxy_class.next();
                self.class_name
                    .borrow_mut()
                    .set_value(self.galaxy_class.name());
                self.class_name
                    .borrow_mut()
                    .positionate(ctx, window::get_size(ctx));
                None
            }
            "randomize" => {
                // TODO: randomize world name
                self.galaxy_class = rand::thread_rng().sample(Standard);
                self.class_name
                    .borrow_mut()
                    .set_value(self.galaxy_class.name());
                self.class_name
                    .borrow_mut()
                    .positionate(ctx, window::get_size(ctx));
                self.seed_input
                    .borrow_mut()
                    .set_value(random_seed().as_str());
                None
            }
            "preview" => {
                let mut hasher = DefaultHasher::new();
                let seed = self.seed_input.borrow().value();
                seed.hash(&mut hasher);
                let seed = hasher.finish();
                let name = self.name_input.borrow().value();
                let size = self.galaxy_size.into();
                self.preview.borrow_mut().redraw(
                    ctx,
                    size,
                    galaxy_generator::generate(seed, size, self.galaxy_class),
                    name.as_str(),
                );
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
                    // TODO: save galaxy size and types
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
