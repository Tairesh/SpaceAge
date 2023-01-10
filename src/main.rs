// #![windows_subsystem = "windows"]

mod app;
mod ascii;
mod assets;
mod astro;
mod colors;
mod data;
mod fov;
mod game;
mod human;
mod input;
mod savefile;
mod scenes;
mod settings;
mod sprites;
mod window;

const TITLE: &str = "Space Age";
const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    env!("SPACEAGE_VERSION_POSTFIX")
);

fn main() -> tetra::Result {
    window::create_context(format!("{} {}", TITLE, VERSION))?.run(app::App::new)
}
