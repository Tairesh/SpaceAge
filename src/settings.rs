use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

const PATH: &str = "./settings.json";
static INSTANCE: OnceCell<Mutex<Settings>> = OnceCell::new();

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Window {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            fullscreen: false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Debug {
    pub show_fps: bool,
    // TODO: debug log, backtrace, god-mode, etc.
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Game {
    pub repeat_interval: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            repeat_interval: 125,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Settings {
    pub window: Window,
    pub debug: Debug,
    pub game: Game,
}

impl Settings {
    pub fn instance() -> MutexGuard<'static, Settings> {
        INSTANCE
            .get_or_init(|| Mutex::new(load(PATH)))
            .lock()
            .expect("CATS: ALL YOUR SETTINGS ARE BELONGS TO US")
    }

    pub fn save(&mut self) {
        self.validate();
        save(self, PATH);
    }

    pub fn validate(&mut self) {
        self.window.width = self.window.width.clamp(800, 1920);
        self.window.height = self.window.height.clamp(600, 1280);
        self.game.repeat_interval = self.game.repeat_interval.clamp(1, 1000);
    }
}

fn load_from_file(path: &'static str) -> Result<Settings, ()> {
    let path = Path::new(path);
    if !path.is_file() {
        return Err(());
    }
    let file = File::open(path).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|_| ())
        .map(|mut settings: Settings| {
            settings.validate();
            settings
        })
}

fn load(path: &'static str) -> Settings {
    load_from_file(path).unwrap_or_else(|_| {
        let settings = Settings::default();
        save(&settings, path);
        settings
    })
}

fn save(settings: &Settings, path: &'static str) {
    serde_json::to_writer(&File::create(Path::new(path)).unwrap(), settings).ok();
}

#[cfg(test)]
mod tests {
    use super::{load, save};

    const TEST_PATH: &str = "./settings-test.json";

    // TODO: this test blocking FS which seems to be a bad idea
    #[test]
    fn test_settings_load_and_save() {
        let mut settings = load(TEST_PATH);
        settings.window.width = 987;
        save(&settings, TEST_PATH);

        let settings = load(TEST_PATH);
        assert_eq!(987, settings.window.width);

        std::fs::remove_file(TEST_PATH).ok();
    }

    #[test]
    fn test_invalid_settings() {
        let mut settings = load(TEST_PATH);
        settings.window.width = 123;
        settings.window.height = 456;
        settings.game.repeat_interval = 0;
        settings.validate();

        assert_eq!(800, settings.window.width);
        assert_eq!(600, settings.window.height);
        assert_eq!(1, settings.game.repeat_interval);
    }
}
