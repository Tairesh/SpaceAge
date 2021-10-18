use crate::assets::Assets;
use crate::colors::Colors;
use crate::data::game_data::GameData;
use crate::human::character::Character;
use crate::human::gender::Gender;
use crate::human::main_hand::MainHand;
use crate::human::skin_tone::SkinTone;
use crate::savefile::SaveFile;
use crate::scenes::{bg, easy_back, Scene, Transition};
use crate::sprites::button::Button;
use crate::sprites::input::TextInput;
use crate::sprites::label::Label;
use crate::sprites::meshy::JustMesh;
use crate::sprites::position::{Horizontal, Position, Vertical};
use crate::sprites::sprite::{Colorize, Draw, Positionate, Sprite, Stringify};
use crate::Vec2;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::input::{Key, KeyModifier};
use tetra::{window, Context, Event};

pub struct CreateCharacter {
    savefile: SaveFile,
    data: Rc<GameData>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    gender_input: Rc<RefCell<TextInput>>,
    age_input: Rc<RefCell<TextInput>>,
    hand_label: Rc<RefCell<Label>>,
    skin_mesh: Rc<RefCell<JustMesh>>,
    skin_label: Rc<RefCell<Label>>,
    main_hand: MainHand,
    skin_tone: SkinTone,
}

impl CreateCharacter {
    pub fn new(savefile: SaveFile, assets: &Assets, data: Rc<GameData>, ctx: &mut Context) -> Self {
        let right_column_width: f32 = 300.0;
        let title = Rc::new(RefCell::new(Label::new(
            "Create new character:",
            assets.fonts.handel32.clone(),
            Colors::ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -220.0 },
            },
        )));
        let subtitle = Rc::new(RefCell::new(Label::new(
            format!("New adventurer in the {} galaxy", savefile.galaxy_name()).as_str(),
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByTop { offset: -200.0 },
            },
        )));
        let name_label = Rc::new(RefCell::new(Label::new(
            "Name:",
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -80.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -126.0 },
            },
        )));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            "",
            right_column_width,
            assets.fonts.handel24.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -120.0 },
            },
        )));
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "Name shall not be empty!",
            assets.fonts.consolab18.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0 - 70.0,
                },
                y: Vertical::AtWindowCenterByBottom { offset: -151.0 },
            },
        )));
        let gender_label = Rc::new(RefCell::new(Label::new(
            "Gender:",
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -80.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -56.0 },
            },
        )));
        let gender_left = Rc::new(RefCell::new(Button::new(
            vec![],
            "<",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                y: Vertical::AtWindowCenterByCenter { offset: -50.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("gender:left".to_string()),
        )));
        let gender_input = Rc::new(RefCell::new(TextInput::new(
            "Female",
            right_column_width - 80.0,
            assets.fonts.handel24.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0 - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: -50.0 },
            },
        )));
        let gender_right = Rc::new(RefCell::new(Button::new(
            vec![],
            ">",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: right_column_width - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: -50.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("gender:right".to_string()),
        )));
        let age_label = Rc::new(RefCell::new(Label::new(
            "Age:",
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -80.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 14.0 },
            },
        )));
        let age_left = Rc::new(RefCell::new(Button::new(
            vec![],
            "-",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 20.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("age:left".to_string()),
        )));
        let age_input = Rc::new(RefCell::new(TextInput::int(
            18,
            (15, 199),
            right_column_width - 80.0,
            assets.fonts.handel24.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0 - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 20.0 },
            },
        )));
        let age_right = Rc::new(RefCell::new(Button::new(
            vec![],
            "+",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: right_column_width - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 20.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("age:right".to_string()),
        )));
        let hand_title = Rc::new(RefCell::new(Label::new(
            "Main hand:",
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -80.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 84.0 },
            },
        )));
        let hand_left = Rc::new(RefCell::new(Button::new(
            vec![],
            "<",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 90.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("hand:left".to_string()),
        )));
        let main_hand = MainHand::Right;
        let hand_label = Rc::new(RefCell::new(Label::new(
            main_hand.name(),
            assets.fonts.handel24.clone(),
            Colors::ORANGE_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0 - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 84.0 },
            },
        )));
        let hand_right = Rc::new(RefCell::new(Button::new(
            vec![],
            ">",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: right_column_width - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 90.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("hand:right".to_string()),
        )));
        let skin_title = Rc::new(RefCell::new(Label::new(
            "Skin tone:",
            assets.fonts.handel24.clone(),
            Colors::ORANGE,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -80.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 154.0 },
            },
        )));
        let skin_left = Rc::new(RefCell::new(Button::new(
            vec![],
            "<",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                y: Vertical::AtWindowCenterByCenter { offset: 160.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("skin:left".to_string()),
        )));
        let skin_tone = SkinTone::WarmIvory;
        let skin_mesh = Rc::new(RefCell::new(JustMesh::new(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, right_column_width - 80.0, 42.0),
                BorderRadii::new(5.0),
            )
            .unwrap(),
            Some(skin_tone.into()),
            Vec2::new(right_column_width - 80.0, 42.0),
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0 - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 160.0 },
            },
        )));
        let skin_label = Rc::new(RefCell::new(Label::new(
            skin_tone.name(),
            assets.fonts.handel24.clone(),
            skin_tone.text_color(),
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: right_column_width / 2.0 - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 154.0 },
            },
        )));
        let skin_right = Rc::new(RefCell::new(Button::new(
            vec![],
            ">",
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: right_column_width - 70.0,
                },
                y: Vertical::AtWindowCenterByCenter { offset: 160.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("skin:right".to_string()),
        )));
        let randomize_btn = Rc::new(RefCell::new(Button::new(
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -100.0 },
                y: Vertical::AtWindowCenterByTop { offset: 220.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("randomize".to_string()),
        )));
        let randomize_size = randomize_btn.borrow_mut().calc_size(ctx);
        let back_btn = Rc::new(RefCell::new(Button::new(
            vec![(Key::Escape, None)],
            "[Esc] Back",
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -110.0 },
                y: Vertical::AtWindowCenterByTop { offset: 220.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::Pop,
        )));
        // TODO: add next stage for selecting scenario and ship name
        let create_btn = Rc::new(RefCell::new(Button::new(
            vec![(Key::Enter, Some(KeyModifier::Alt))],
            "[Alt+Enter] Create",
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x - 90.0,
                },
                y: Vertical::AtWindowCenterByTop { offset: 220.0 },
            },
            assets.fonts.consolab18.clone(),
            Transition::CustomEvent("create".to_string()),
        )));

        Self {
            savefile,
            data,
            sprites: vec![
                bg(assets),
                title,
                subtitle,
                name_label,
                name_input.clone(),
                name_empty.clone(),
                gender_label,
                gender_left,
                gender_input.clone(),
                gender_right,
                age_label,
                age_left,
                age_input.clone(),
                age_right,
                hand_title,
                hand_left,
                hand_label.clone(),
                hand_right,
                skin_title,
                skin_left,
                skin_mesh.clone(),
                skin_label.clone(),
                skin_right,
                back_btn,
                randomize_btn,
                create_btn,
            ],
            name_input,
            name_empty,
            gender_input,
            age_input,
            hand_label,
            skin_mesh,
            skin_label,
            main_hand,
            skin_tone,
        }
    }
}

impl Scene for CreateCharacter {
    fn update(&mut self, _ctx: &mut Context, _focused: bool) -> Transition {
        let mut name_error = self.name_empty.borrow_mut();
        if !self.name_input.borrow().danger() && name_error.visible() {
            name_error.set_visible(false);
        }
        Transition::DoNothing
    }
    fn event(&mut self, _ctx: &mut Context, event: Event, focused: bool) -> Transition {
        easy_back(event, focused).unwrap_or(Transition::DoNothing)
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: &str) -> Option<Transition> {
        match event {
            "create" => {
                let name = self.name_input.borrow().value();
                if name.is_empty() {
                    self.name_input.borrow_mut().set_danger(true);
                    self.name_empty.borrow_mut().set_visible(true);
                    return None;
                }

                let gender = self.gender_input.borrow().value();
                let age = self.age_input.borrow().value().parse::<u8>().unwrap();
                let character = Character::new(
                    name,
                    Gender::from(gender),
                    age,
                    self.main_hand,
                    self.skin_tone,
                );
                self.savefile.set_character(character);
                Some(Transition::CreateWorld(self.savefile.clone()))
            }
            "randomize" => {
                let mut rng = rand::thread_rng();
                let window_size = window::get_size(ctx);
                let character = Character::random(&mut rng, &self.data);
                self.name_input.borrow_mut().set_value(character.name);
                self.gender_input.borrow_mut().set_value(character.gender);
                self.age_input
                    .borrow_mut()
                    .set_value(character.age.to_string());
                self.main_hand = character.main_hand;
                let mut hand = self.hand_label.borrow_mut();
                hand.set_value(self.main_hand.name());
                hand.positionate(ctx, window_size);
                self.skin_tone = character.skin_tone;
                let mut skin = self.skin_label.borrow_mut();
                skin.set_value(self.skin_tone.name());
                skin.set_color(self.skin_tone.text_color());
                skin.positionate(ctx, window_size);
                self.skin_mesh.borrow_mut().set_color(self.skin_tone);
                None
            }
            "gender:left" | "gender:right" => {
                let mut input = self.gender_input.borrow_mut();
                let value = input.value();
                input.set_value(if value == "Male" { "Female" } else { "Male" });
                None
            }
            "age:left" | "age:right" => {
                let mut input = self.age_input.borrow_mut();
                if let Ok(mut value) = input.value().parse::<u8>() {
                    if event == "age:right" {
                        value += 1;
                    } else if value > 0 {
                        value -= 1;
                    }
                    input.set_value(value.to_string());
                }
                None
            }
            "hand:left" | "hand:right" => {
                let mut label = self.hand_label.borrow_mut();
                self.main_hand = if event == "hand:right" {
                    self.main_hand.next()
                } else {
                    self.main_hand.prev()
                };
                label.set_value(self.main_hand.name());
                label.positionate(ctx, window::get_size(ctx));
                None
            }
            "skin:left" | "skin:right" => {
                self.skin_tone = if event == "skin:right" {
                    self.skin_tone.next()
                } else {
                    self.skin_tone.prev()
                };
                self.skin_mesh.borrow_mut().set_color(self.skin_tone);
                let mut label = self.skin_label.borrow_mut();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, window::get_size(ctx));
                None
            }
            _ => unreachable!(),
        }
    }
}
