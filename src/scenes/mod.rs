mod create_world;
mod empty;
pub mod main_menu;
mod settings;

use crate::assets::Assets;
use crate::scenes::create_world::CreateWorld;
use crate::scenes::empty::Empty;
use crate::scenes::main_menu::MainMenu;
use crate::scenes::settings::SettingsScene;
use crate::settings::{Settings, WindowMode};
use crate::sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::{Context, Event};

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum GameScene {
    MainMenu,
    Empty,
    Settings,
    CreateWorld,
}

impl GameScene {
    pub fn into_scene(
        self,
        assets: &Assets,
        settings: &Settings,
        ctx: &mut Context,
    ) -> Box<dyn Scene> {
        match self {
            GameScene::MainMenu => Box::new(MainMenu::new(assets)),
            GameScene::Empty => Box::new(Empty {}),
            GameScene::Settings => Box::new(SettingsScene::new(assets, settings, ctx)),
            GameScene::CreateWorld => Box::new(CreateWorld::new(assets, ctx)),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Transition {
    DoNothing,
    Push(GameScene),
    Pop,
    Pop2,               // two times
    Replace(GameScene), // pop and push
    CustomEvent(String),
    ChangeWindowMode(WindowMode),
    Quit,
}

pub(crate) fn easy_back(event: Event) -> Option<Transition> {
    if matches!(
        event,
        Event::MouseButtonPressed {
            button: MouseButton::X1,
        } | Event::KeyPressed { key: Key::Escape }
            | Event::KeyPressed {
                key: Key::Backspace
            }
    ) {
        Some(Transition::Pop)
    } else {
        None
    }
}

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context) -> Transition {
        Transition::DoNothing
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event) -> Transition {
        Transition::DoNothing
    }
    fn draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context) {}
    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: &str) -> Option<Transition> {
        None
    }
}
