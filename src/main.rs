#![windows_subsystem = "windows"]

mod action;
mod assets;
mod avatar;
mod colors;
mod direction;
mod game;
mod geometry;
mod human;
mod map;
mod savefile;
mod scenes;
mod settings;
mod sprites;
mod world;

extern crate chrono;
extern crate num_enum;
extern crate rand;
extern crate serde;
extern crate tetra;

use crate::game::Game;
use crate::settings::Settings;
use tetra::graphics::ImageData;
use tetra::{window, ContextBuilder};

const TITLE: &str = "Space Age";
const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    env!("SPACEAGE_VERSION_POSTFIX")
);

type Vec2 = tetra::math::Vec2<f32>;
type Rect = tetra::math::Rect<f32, f32>;

fn main() -> tetra::Result {
    let settings = Settings::load()?;
    let title = format!("{} {}", TITLE, VERSION);
    let mut ctx = ContextBuilder::new(title.clone(), settings.width as i32, settings.height as i32);
    ctx.show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true);
    if settings.fullscreen {
        ctx.fullscreen(true);
    }
    let mut ctx = ctx.build()?;
    let mut icon = ImageData::from_file_data(include_bytes!("../inc/img/icon.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;
    window::set_minimum_size(&mut ctx, 1024, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;

    ctx.run(|ctx| Ok(Game::new(ctx, settings, title)))
}