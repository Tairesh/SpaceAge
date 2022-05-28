use crate::assets::Assets;
use crate::data::game_data::GameData;
use crate::game::world::World;
use crate::scenes::main_menu::MainMenu;
use crate::scenes::{GameScene, Scene, Transition};
use crate::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::{time, window, Context, Event, Result, State};

pub struct App {
    scenes: Vec<Box<dyn Scene>>,
    pub settings: Rc<RefCell<Settings>>,
    pub assets: Rc<Assets>,
    pub data: Rc<GameData>,
    pub world: Option<Rc<RefCell<World>>>,
    default_title: String,
    current_fps: u8,
}

impl App {
    pub fn new(ctx: &mut Context, settings: Settings) -> Result<Self> {
        let assets = Rc::new(Assets::load(ctx)?);
        let data = Rc::new(GameData::load());
        let mut game = Self {
            scenes: vec![Box::new(MainMenu::new(&assets))],
            settings: Rc::new(RefCell::new(settings)),
            assets,
            data,
            default_title: window::get_title(ctx).to_string(),
            current_fps: 60,
            world: None,
        };
        game.on_open(ctx);
        Ok(game)
    }

    fn on_open(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.current_scene() {
            scene.on_open(ctx);
        }
        self.on_resize(ctx);
    }

    fn on_resize(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.current_scene() {
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
        if self.settings.borrow().show_fps {
            let fps = time::get_fps(ctx).round() as u8;
            if fps != self.current_fps {
                window::set_title(ctx, format!("{} ({} FPS)", self.default_title, fps));
                self.current_fps = fps;
            }
        }
    }

    fn pop_scene(&mut self, ctx: &mut Context) {
        self.scenes.pop();
        self.on_open(ctx);
    }

    fn replace_scene(&mut self, ctx: &mut Context, scene: GameScene) {
        self.scenes.pop();
        self.push_scene(ctx, scene);
    }

    fn push_scene(&mut self, ctx: &mut Context, scene: GameScene) {
        self.scenes.push(scene.into_scene(self, ctx));
        self.on_open(ctx);
    }

    fn current_scene(&mut self) -> Option<&mut Box<dyn Scene>> {
        self.scenes.last_mut()
    }

    fn transit(&mut self, ctx: &mut Context, transition: Transition) {
        match transition {
            Transition::DoNothing => {}
            Transition::CreateWorld(savefile) => {
                let world = World::create(&savefile, &self.data);
                world.save();
                self.world = Some(Rc::new(RefCell::new(world)));
                self.replace_scene(ctx, GameScene::ShipWalk);
            }
            Transition::LoadWorld(savefile) => {
                if let Ok(world) = savefile.load_world() {
                    self.world = Some(Rc::new(RefCell::new(world)));
                    self.replace_scene(ctx, GameScene::ShipWalk);
                } else {
                    // TODO: show message and go to main menu
                    panic!("Can't load world: {:?}", savefile.path)
                }
            }
            Transition::UnloadWorld => {
                if let Some(world) = &self.world {
                    world.borrow().save();
                }
                self.scenes.clear();
                self.push_scene(ctx, GameScene::MainMenu);
            }
            Transition::Push(s) => {
                self.push_scene(ctx, s);
            }
            Transition::Pop => {
                self.pop_scene(ctx);
            }
            Transition::Replace(s) => {
                self.replace_scene(ctx, s);
            }
            Transition::CustomEvent(str) => {
                if let Some(scene) = self.current_scene() {
                    if let Some(t) = scene.custom_event(ctx, str.as_str()) {
                        self.transit(ctx, t);
                    }
                }
            }
            Transition::Quit => {
                window::quit(ctx);
            }
        }
    }
}

impl State for App {
    fn update(&mut self, ctx: &mut Context) -> Result {
        self.show_fps(ctx);
        let transition = if let Some(scene) = self.current_scene() {
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
        if let Some(scene) = self.current_scene() {
            if let Some(sprites) = scene.sprites() {
                for sprite in sprites.iter() {
                    let mut sprite = sprite.borrow_mut();
                    if sprite.visible() {
                        sprite.draw(ctx);
                    }
                }
            }
            scene.draw(ctx);
        }
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                let mut settings = self.settings.borrow_mut();
                settings.show_fps = !settings.show_fps;
                if !settings.show_fps {
                    window::set_title(ctx, &self.default_title);
                }
            }
            Event::Resized { width, height } => {
                let mut settings = self.settings.borrow_mut();
                if !settings.fullscreen {
                    settings.width = width;
                    settings.height = height;
                    settings.validate();
                }
                drop(settings);
                self.on_resize(ctx);
            }
            _ => {}
        }
        if let Some(scene) = self.current_scene() {
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

impl Drop for App {
    fn drop(&mut self) {
        self.settings.borrow_mut().save();
        if let Some(world) = &self.world {
            world.borrow().save();
        }
    }
}
