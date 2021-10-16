use crate::assets::Assets;
use crate::data::game_data::GameData;
use crate::scenes::main_menu::MainMenu;
use crate::scenes::{GameScene, Scene, Transition};
use crate::settings::{Settings, WindowMode};
use crate::things::world::World;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::window::WindowPosition;
use tetra::{time, window, Context, Event, Result, State};

pub struct Game {
    scenes: Vec<Box<dyn Scene>>,
    settings: Settings,
    assets: Assets,
    data: GameData,
    world: Option<Rc<RefCell<World>>>,
    default_title: String,
    current_fps: u8,
}

impl Game {
    pub fn new(ctx: &mut Context, settings: Settings, default_title: String) -> Self {
        let assets = Assets::new(ctx);
        let data = GameData::load();
        let mut game = Self {
            scenes: vec![Box::new(MainMenu::new(&assets))],
            settings,
            assets,
            data,
            default_title,
            current_fps: 60,
            world: None,
        };
        game.on_open(ctx);
        game
    }

    fn on_open(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.on_open(ctx);
        }
        self.on_resize(ctx);
    }

    fn on_resize(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.scenes.last_mut() {
            let window_size = window::get_size(ctx);
            if let Some(sprites) = scene.sprites() {
                for sprite in sprites.iter() {
                    sprite.borrow_mut().positionate(ctx, window_size);
                }
            }
            scene.on_resize(ctx);
        }
    }

    fn show_fps(&mut self, ctx: &mut Context) {
        if self.settings.show_fps {
            let fps = time::get_fps(ctx).round() as u8;
            if fps != self.current_fps {
                window::set_title(ctx, format!("{} ({} FPS)", self.default_title, fps));
                self.current_fps = fps;
            }
        }
    }

    // TODO: implement replace_scene and pop_scene
    fn push_scene(&mut self, ctx: &mut Context, scene: GameScene) {
        self.scenes
            .push(scene.into_scene(&self.world, &self.assets, &self.settings, ctx));
        self.on_open(ctx);
    }

    fn transit(&mut self, ctx: &mut Context, transition: Transition) {
        match transition {
            Transition::DoNothing => {}
            Transition::CreateWorld(savefile) => {
                let world = World::create(&savefile, &self.data);
                world.save();
                self.world = Some(Rc::new(RefCell::new(world)));
                self.scenes.pop();
                self.push_scene(ctx, GameScene::ShipWalk);
            }
            Transition::LoadWorld(savefile) => {
                self.world = Some(Rc::new(RefCell::new(savefile.load_world())));
                self.scenes.pop();
                self.push_scene(ctx, GameScene::ShipWalk);
            }
            Transition::Push(s) => {
                self.push_scene(ctx, s);
            }
            Transition::Pop => {
                self.scenes.pop();
                self.on_open(ctx);
            }
            Transition::Pop2 => {
                self.scenes.pop();
                self.scenes.pop();
                self.on_open(ctx);
            }
            Transition::Replace(s) => {
                self.scenes.pop();
                self.push_scene(ctx, s);
            }
            Transition::CustomEvent(str) => {
                if let Some(t) = self
                    .scenes
                    .last_mut()
                    .unwrap()
                    .custom_event(ctx, str.as_str())
                {
                    self.transit(ctx, t);
                }
            }
            Transition::Quit => {
                window::quit(ctx);
            }
            Transition::ChangeWindowMode(wm) => {
                if self.settings.window_mode() != wm {
                    match wm {
                        WindowMode::Fullscreen => {
                            self.settings.fullscreen = true;
                            window::set_fullscreen(ctx, true).ok();
                        }
                        WindowMode::Window => {
                            self.settings.fullscreen = false;
                            if window::is_fullscreen(ctx) {
                                window::set_fullscreen(ctx, false).ok();
                            }
                            window::set_decorated(ctx, true);
                            window::set_size(
                                ctx,
                                self.settings.width as i32,
                                self.settings.height as i32,
                            )
                            .ok();
                            window::set_position(
                                ctx,
                                WindowPosition::Centered(0),
                                WindowPosition::Centered(0),
                            );
                        }
                    }
                }
            }
        }
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) -> Result {
        self.show_fps(ctx);
        let transition = if let Some(scene) = self.scenes.last_mut() {
            let mut button_clicked = None;
            let focused = scene
                .sprites()
                .map(|sprites| sprites.iter().any(|s| s.borrow().focused()))
                .unwrap_or(false);
            if let Some(sprites) = scene.sprites() {
                let mut blocked = Vec::with_capacity(sprites.len());
                for sprite in sprites.iter().rev() {
                    let mut sprite = sprite.borrow_mut();
                    if let Some(transition) = sprite.update(ctx, focused, &blocked) {
                        button_clicked = Some(transition);
                    }
                    if sprite.visible() && sprite.block_mouse() {
                        blocked.push(sprite.rect());
                    }
                }
            }
            if let Some(t) = button_clicked {
                t
            } else {
                scene.update(ctx, focused)
            }
        } else {
            Transition::Quit
        };
        self.transit(ctx, transition);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        if let Some(scene) = self.scenes.last_mut() {
            scene.draw(ctx);
        }
        if let Some(scene) = self.scenes.last_mut() {
            if let Some(sprites) = scene.sprites() {
                for sprite in sprites.iter() {
                    let mut sprite = sprite.borrow_mut();
                    if sprite.visible() {
                        sprite.draw(ctx);
                    }
                }
            }
        }
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                self.settings.show_fps = !self.settings.show_fps;
                if !self.settings.show_fps {
                    window::set_title(ctx, &self.default_title);
                }
            }
            Event::Resized { width, height } => {
                if !self.settings.fullscreen {
                    self.settings.width = width as u32;
                    self.settings.height = height as u32;
                    self.settings.validate();
                }
                self.on_resize(ctx);
            }
            _ => {}
        }
        if let Some(scene) = self.scenes.last_mut() {
            let focused = scene
                .sprites()
                .map(|sprites| sprites.iter().any(|s| s.borrow().focused()))
                .unwrap_or(false);
            let t = scene.event(ctx, event, focused);
            self.transit(ctx, t);
        }
        Ok(())
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.settings.save();
        if let Some(world) = &self.world {
            world.borrow().save();
        }
    }
}
