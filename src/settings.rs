use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const PATH: &str = "settings.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
    pub show_fps: bool,
    pub music_enabled: bool,
    pub music_volume: u8,
    pub repeat_interval: u32,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            width: 1024,
            height: 768,
            fullscreen: false,
            show_fps: false,
            music_enabled: true,
            music_volume: 64,
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
        if self.width < 800 {
            self.width = 800;
        }
        if self.width > 1920 {
            self.width = 1920;
        }
        if self.height < 600 {
            self.height = 600;
        }
        if self.height > 1280 {
            self.height = 1280;
        }
        if self.music_volume > 128 {
            self.music_volume = 128;
        }
        self
    }
}
