use crate::astro::galaxy_generator;
use crate::avatar::Avatar;
use crate::world::{World, WorldMeta};
use crate::VERSION;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
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
pub enum SaveError {
    SystemError(String),
    SerializeError(String),
    FileExists,
}

impl From<serde_json::Error> for SaveError {
    fn from(e: serde_json::Error) -> Self {
        SaveError::SerializeError(e.to_string())
    }
}

impl From<std::io::Error> for SaveError {
    fn from(e: std::io::Error) -> Self {
        SaveError::SystemError(e.to_string())
    }
}

fn make_data(savefile: &SaveFile, world: Option<&World>) -> Result<String, SaveError> {
    let data = [
        serde_json::to_string(savefile).map_err(SaveError::from)?,
        if let Some(world) = world {
            serde_json::to_string(&world.quadrants)
        } else {
            // TODO: Galaxy structure, not vector of ints
            serde_json::to_string(&galaxy_generator::generate_quadrants(
                savefile.world_meta.seed,
                savefile.world_meta.size.into(),
                savefile.world_meta.class,
            ))
        }
        .map_err(SaveError::from)?,
        // TODO: ship, other units, sectors data
    ]
    .join("\n");
    Ok(data)
}

fn make_dir() -> Result<(), SaveError> {
    let dir = Path::new("save");
    if !dir.exists() {
        std::fs::create_dir(dir).map_err(SaveError::from)?;
    }
    Ok(())
}

pub fn create(world_meta: WorldMeta) -> Result<(), SaveError> {
    let savefile: SaveFile = world_meta.into();
    make_dir()?;
    if savefile.path.is_file() {
        return Err(SaveError::FileExists);
    }
    let mut file = File::create(&savefile.path).map_err(SaveError::from)?;
    file.write_all(make_data(&savefile, None)?.as_bytes())
        .map_err(|e| e.into())
}

pub fn load(path: &Path) -> Option<SaveFile> {
    let file = File::open(path).ok()?;
    let mut lines = BufReader::new(&file).lines();
    let meta = lines.next()?.ok()?;
    serde_json::from_str(meta.as_str())
        .ok()
        .map(|s: SaveFile| s.with_path(path))
}

pub fn save(world: &World) -> Result<(), SaveError> {
    let savefile: SaveFile = world.into();
    make_dir()?;
    let mut file = File::create(&savefile.path).map_err(SaveError::from)?;
    file.write_all(make_data(&savefile, Some(world))?.as_bytes())
        .map_err(SaveError::from)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveFile {
    #[serde(skip)]
    pub path: PathBuf,
    pub version: String,
    pub time: SystemTime,
    world_meta: WorldMeta, // TODO: save whole World instead
    avatar: Option<Avatar>,
}

impl SaveFile {
    // there are 3 ways to create SaveFile
    // 1. WorldMeta::into(), uses in create()
    // 2. load(&Path) uses in savefiles() and LoadWorld scene for loading only first string
    // 3. World::into(), uses in save()

    fn with_path(mut self, path: &Path) -> Self {
        self.path = path.into();
        self
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

    pub fn as_world(&self) -> World {
        self.into()
    }
}

impl Eq for SaveFile {}

impl PartialEq<Self> for SaveFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd<Self> for SaveFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SaveFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl From<WorldMeta> for SaveFile {
    fn from(world_meta: WorldMeta) -> Self {
        let file_name = world_meta.name.replace(" ", "_");
        Self {
            path: ["save", (file_name + ".save").as_str()].iter().collect(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            world_meta,
            avatar: None,
        }
    }
}

impl From<&SaveFile> for World {
    fn from(savefile: &SaveFile) -> Self {
        // TODO: too many unwrap() here
        let file = File::open(&savefile.path).unwrap();
        let mut lines = BufReader::new(&file).lines();
        let sectors = serde_json::from_str(lines.nth(1).unwrap().ok().unwrap().as_str()).unwrap();
        World::new(
            savefile.path.clone(),
            savefile.world_meta.clone(),
            sectors,
            savefile.avatar.as_ref().unwrap().clone(),
        )
    }
}

impl From<&World> for SaveFile {
    fn from(world: &World) -> Self {
        SaveFile {
            path: world.path.clone(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            world_meta: world.meta.clone(),
            avatar: Some(world.avatar.clone()),
        }
    }
}

pub fn savefiles() -> Vec<SaveFile> {
    let path = Path::new("save");
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            let p = p.unwrap().path();
            if let Some(s) = load(&p) {
                files.push(s);
            }
        }
    }
    files.sort();
    files
}
