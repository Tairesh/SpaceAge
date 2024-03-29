use crate::astro::galaxy::{Galaxy, GalaxyMeta};
use crate::game::world::World;
use crate::human::character::Character;
use crate::VERSION;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// TODO: move it away, like in Necromanzer

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

pub fn savefiles() -> Vec<SaveFile> {
    let path = Path::new("save");
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            let p = p.unwrap().path();
            if let Some(s) = load(&p) {
                // TODO: some implementation for invalid (old) savefiles
                files.push(s);
            }
        }
    }
    files.sort();
    files.reverse();
    files
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

#[derive(Debug)]
pub enum LoadError {
    SystemError(String),
    DeserializeError(String),
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self {
        LoadError::DeserializeError(e.to_string())
    }
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::SystemError(e.to_string())
    }
}

fn make_data(savefile: &SaveFile, world: Option<&World>) -> Result<String, SaveError> {
    let mut data = vec![serde_json::to_string(savefile).map_err(SaveError::from)?];
    if let Some(world) = world {
        data.push(serde_json::to_string(&world.galaxy).map_err(SaveError::from)?);
        data.push(serde_json::to_string(&world.avatar).map_err(SaveError::from)?);
        data.push(serde_json::to_string(&world.ship).map_err(SaveError::from)?);
        // TODO: other units, sectors data
    } else {
        data.push(
            serde_json::to_string(&Galaxy::from(savefile.galaxy_meta.clone()))
                .map_err(SaveError::from)?,
        );
    }
    let data = data.join("\n");
    Ok(data)
}

fn make_dir() -> Result<(), SaveError> {
    let dir = Path::new("save");
    if !dir.exists() {
        std::fs::create_dir(dir).map_err(SaveError::from)?;
    }
    Ok(())
}

pub fn create(galaxy_meta: GalaxyMeta) -> Result<(), SaveError> {
    let savefile: SaveFile = galaxy_meta.into();
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
    pub current_tick: u128,
    pub character: Option<Character>,
    galaxy_meta: GalaxyMeta,
}

impl SaveFile {
    // there are 3 ways to create SaveFile
    // 1. GalaxyMeta::into(), uses in create()
    // 2. load(&Path) uses in savefiles() and LoadWorld scene for loading only first string
    // 3. World::into(), uses in save()

    fn with_path(mut self, path: &Path) -> Self {
        self.path = path.into();
        self
    }

    pub fn galaxy_name(&self) -> &str {
        self.galaxy_meta.name.as_str()
    }

    pub fn character_name(&self) -> &str {
        if let Some(character) = &self.character {
            character.name.as_str()
        } else {
            "no character"
        }
    }

    pub fn has_character(&self) -> bool {
        self.character.is_some()
    }

    pub fn set_character(&mut self, character: Character) -> &mut Self {
        self.character = Some(character);
        self
    }

    pub fn load_galaxy(&self) -> Result<Galaxy, LoadError> {
        let file = File::open(&self.path).map_err(LoadError::from)?;
        let mut lines = BufReader::new(&file).lines();
        serde_json::from_str(lines.nth(1).unwrap().map_err(LoadError::from)?.as_str())
            .map_err(LoadError::from)
    }

    pub fn load_world(&self) -> Result<World, LoadError> {
        let file = File::open(&self.path).map_err(LoadError::from)?;
        let mut lines = BufReader::new(&file).lines();
        let galaxy = serde_json::from_str(lines.nth(1).unwrap().map_err(LoadError::from)?.as_str())
            .map_err(LoadError::from)?;
        let avatar = serde_json::from_str(lines.next().unwrap().map_err(LoadError::from)?.as_str())
            .map_err(LoadError::from)?;
        let ship = serde_json::from_str(lines.next().unwrap().map_err(LoadError::from)?.as_str())
            .map_err(LoadError::from)?;
        Ok(World::new(
            self.path.clone(),
            galaxy,
            avatar,
            ship,
            self.current_tick,
        ))
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

impl From<GalaxyMeta> for SaveFile {
    fn from(galaxy_meta: GalaxyMeta) -> Self {
        let file_name = galaxy_meta.name.replace(' ', "_");
        Self {
            path: ["save", (file_name + ".save").as_str()].iter().collect(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            galaxy_meta,
            character: None,
            current_tick: 0,
        }
    }
}

impl From<&World> for SaveFile {
    fn from(world: &World) -> Self {
        SaveFile {
            path: world.path.clone(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            galaxy_meta: world.galaxy.meta.clone(),
            character: Some(world.avatar.character.clone()),
            current_tick: world.current_tick,
        }
    }
}
