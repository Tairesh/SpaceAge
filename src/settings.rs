use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const PATH: &str = "settings.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
    pub window_size: (i32, i32),
    pub fullscreen: bool,
    pub show_fps: bool,
    pub repeat_interval: u32,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            window_size: (1024, 768),
            fullscreen: false,
            show_fps: false,
            repeat_interval: 125,
        }
    }

    pub fn load() -> tetra::Result<Settings> {
        let path = Path::new(PATH);
        let mut settings: Settings;
        if path.is_file() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            settings = serde_json::from_reader(reader).unwrap();
            settings.validate();
        } else {
            settings = Settings::default();
            serde_json::to_writer(&File::create(path).unwrap(), &settings).unwrap();
        }

        Ok(settings)
    }

    pub fn save(&mut self) {
        self.validate();
        serde_json::to_writer(&File::create(Path::new(PATH)).unwrap(), self).unwrap();
    }

    pub fn validate(&mut self) -> &Settings {
        if self.window_size.0 < 800 {
            self.window_size.0 = 800;
        }
        if self.window_size.0 > 1920 {
            self.window_size.0 = 1920;
        }
        if self.window_size.1 < 600 {
            self.window_size.1 = 600;
        }
        if self.window_size.1 > 1280 {
            self.window_size.1 = 1280;
        }
        self
    }
}
