use crate::assets::Assets;
use crate::scenes::main_menu::MainMenu;
use crate::scenes::{Scene, Transition};
use crate::settings::{Settings, WindowMode};
use tetra::input::Key;
use tetra::window::WindowPosition;
use tetra::{time, window, Context, Event, Result, State};

pub struct Game {
    scenes: Vec<Box<dyn Scene>>,
    settings: Settings,
    assets: Assets,
    default_title: String,
    current_fps: u8,
}

impl Game {
    pub fn new(ctx: &mut Context, settings: Settings, default_title: String) -> Self {
        let assets = Assets::new(ctx);
        let mut game = Self {
            scenes: vec![Box::new(MainMenu::new(&assets))],
            settings,
            assets,
            default_title,
            current_fps: 60,
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

    fn transit(&mut self, ctx: &mut Context, transition: Transition) {
        match transition {
            Transition::DoNothing => {}
            Transition::Push(s) => {
                self.scenes
                    .push(s.into_scene(&self.assets, &self.settings, ctx));
                self.on_open(ctx);
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
                self.scenes
                    .push(s.into_scene(&self.assets, &self.settings, ctx));
                self.on_open(ctx);
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
        // TODO: provide a way to get sprites and scene to know if there are focused input selected
        // for example: not calling easy_back() if text input is focused
        let transition = if let Some(scene) = self.scenes.last_mut() {
            let mut button_clicked = None;
            if let Some(sprites) = scene.sprites() {
                for sprite in sprites.iter() {
                    if let Some(transition) = sprite.borrow_mut().update(ctx) {
                        button_clicked = Some(transition);
                    }
                }
            }
            if let Some(t) = button_clicked {
                t
            } else {
                scene.update(ctx)
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
            let t = scene.event(ctx, event);
            self.transit(ctx, t);
        }
        Ok(())
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.settings.save();
    }
}