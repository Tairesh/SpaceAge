mod create_character;
mod create_world;
mod empty;
mod load_world;
pub mod main_menu;
mod settings;
mod ship_walk;

use crate::assets::Assets;
use crate::savefile::SaveFile;
use crate::scenes::create_character::CreateCharacter;
use crate::scenes::create_world::CreateWorld;
use crate::scenes::empty::Empty;
use crate::scenes::load_world::LoadWorld;
use crate::scenes::main_menu::MainMenu;
use crate::scenes::settings::SettingsScene;
use crate::scenes::ship_walk::ShipWalk;
use crate::settings::{Settings, WindowMode};
use crate::sprites::sprite::Sprite;
use crate::world::World;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::{Context, Event};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum GameScene {
    MainMenu,
    Empty,
    Settings,
    CreateWorld,
    LoadWorld,
    CreateCharacter(SaveFile),
    ShipWalk,
}

impl GameScene {
    pub fn into_scene(
        self,
        world: &Option<Rc<RefCell<World>>>,
        assets: &Assets,
        settings: &Settings,
        ctx: &mut Context,
    ) -> Box<dyn Scene> {
        match self {
            GameScene::MainMenu => Box::new(MainMenu::new(assets)),
            GameScene::Empty => Box::new(Empty {}),
            GameScene::Settings => Box::new(SettingsScene::new(assets, settings, ctx)),
            GameScene::CreateWorld => Box::new(CreateWorld::new(assets, ctx)),
            GameScene::LoadWorld => Box::new(LoadWorld::new(assets, ctx)),
            GameScene::CreateCharacter(s) => Box::new(CreateCharacter::new(s, assets, ctx)),
            GameScene::ShipWalk => {
                Box::new(ShipWalk::new(world.as_ref().unwrap().clone(), assets, ctx))
            }
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Transition {
    DoNothing,
    Push(GameScene),
    LoadWorldAndPush(SaveFile, GameScene),
    Pop,
    Pop2,               // two times
    Replace(GameScene), // pop and push
    CustomEvent(String),
    ChangeWindowMode(WindowMode),
    Quit,
}

pub(crate) fn easy_back(event: Event, focused: bool) -> Option<Transition> {
    if focused {
        return None;
    }
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
    fn update(&mut self, _ctx: &mut Context, _focused: bool) -> Transition {
        Transition::DoNothing
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event, _focused: bool) -> Transition {
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
