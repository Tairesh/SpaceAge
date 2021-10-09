use crate::astro::galaxy_generator;
use crate::avatar::Avatar;
use crate::world::{World, WorldMeta};
use crate::VERSION;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn savefiles_exists() -> bool {
    let path = Path::new("save");
    path.read_dir()
        .map(|mut read_dir| {
            read_dir.any(|entry| {
                entry
                    .map(|entry| {
                        entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                            && entry
                                .path()
                                .extension()
                                .map(|ext| ext == "save")
                                .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

pub fn delete(path: &Path) {
    if path.exists() {
        std::fs::remove_file(path).ok();
    }
}

#[derive(Debug)]
pub enum CreateFileError {
    SystemError(String),
    FileExists,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveFileMeta {
    #[serde(skip)]
    pub path: PathBuf,
    pub version: String,
    pub time: SystemTime,
    world_meta: WorldMeta,
    avatar: Option<Avatar>,
}

impl SaveFileMeta {
    pub fn new(world_meta: WorldMeta) -> Self {
        let file_name = world_meta.name.replace(" ", "_");
        let path: PathBuf = ["save", (file_name + ".save").as_str()].iter().collect();
        Self {
            path,
            version: VERSION.to_string(),
            time: SystemTime::now(),
            world_meta,
            avatar: None,
        }
    }

    pub fn create(&mut self) -> Result<(), CreateFileError> {
        let dir = Path::new("save");
        if !dir.exists() {
            std::fs::create_dir(dir).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        }
        if self.path.is_file() {
            return Err(CreateFileError::FileExists);
        }
        self.time = SystemTime::now();
        let mut file =
            File::create(&self.path).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        let data = [
            serde_json::to_string(self).map_err(|e| CreateFileError::SystemError(e.to_string()))?,
            serde_json::to_string(&galaxy_generator::generate(
                self.world_meta.seed,
                self.world_meta.size.into(),
                self.world_meta.class,
            ))
            .map_err(|e| CreateFileError::SystemError(e.to_string()))?,
        ]
        .join("\n");
        file.write_all(data.as_bytes())
            .map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        let file = File::open(&self.path).ok().unwrap();
        let lines = BufReader::new(&file).lines();
        let mut data: Vec<String> = lines.skip(1).map(Result::unwrap).collect();
        self.time = SystemTime::now();
        self.version = VERSION.to_string();
        data.insert(0, serde_json::to_string(self).map_err(|e| e.to_string())?);
        let mut file = File::create(&self.path).map_err(|e| e.to_string())?;
        file.write_all(data.join("\n").as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.path = path.into();
        self
    }

    pub fn load(path: &Path) -> Option<Self> {
        let file = File::open(path).ok()?;
        let mut lines = BufReader::new(&file).lines();
        let meta = lines.next()?.ok()?;
        serde_json::from_str(meta.as_str())
            .ok()
            .map(|s: SaveFileMeta| s.with_path(path))
    }

    pub fn name(&self) -> &str {
        self.world_meta.name.as_str()
    }

    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    pub fn set_avatar(&mut self, avatar: Avatar) -> &mut Self {
        self.avatar = Some(avatar);
        self
    }

    // TODO: wrap with Result<>
    pub fn load_sectors(&self) -> Vec<u32> {
        let file = File::open(&self.path).unwrap();
        let mut lines = BufReader::new(&file).lines();
        let data = lines.nth(1).unwrap().ok().unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    }
}

impl From<&SaveFileMeta> for World {
    fn from(meta: &SaveFileMeta) -> Self {
        let sectors = meta.load_sectors();
        World::new(
            meta.path.clone(),
            meta.world_meta.clone(),
            sectors,
            meta.avatar.as_ref().unwrap().clone(),
        )
    }
}

impl From<&World> for SaveFileMeta {
    fn from(world: &World) -> Self {
        SaveFileMeta {
            path: world.path.clone(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            world_meta: world.meta.clone(),
            avatar: Some(world.avatar.clone()),
        }
    }
}

pub fn savefiles() -> Vec<SaveFileMeta> {
    let path = Path::new("save");
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            let p = p.unwrap().path();
            if let Some(s) = SaveFileMeta::load(&p) {
                dbg!(&s);
                files.push(s);
            }
        }
    }
    files.sort_by(|s1, s2| s2.time.cmp(&s1.time));
    files
}

// TODO: wrap with Result<>
#[allow(dead_code)]
pub fn save_world(world: &World) {
    let savefile: SaveFileMeta = world.into();
    let data = [
        serde_json::to_string(&savefile).unwrap(),
        serde_json::to_string(&world.sectors).unwrap(),
    ];
    let mut file = File::create(&savefile.path).unwrap();
    file.write_all(data.join("\n").as_bytes()).ok();
}
