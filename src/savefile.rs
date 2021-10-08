#![allow(dead_code)]
use crate::astro::galaxy_class::GalaxyClass;
use crate::astro::galaxy_size::GalaxySize;
use crate::avatar::Avatar;
use crate::world::{World, WorldMeta};
use crate::VERSION;
use std::collections::hash_map::DefaultHasher;
use std::fs::{create_dir, remove_file, File};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct SaveFile {
    pub path: PathBuf,
    pub version: String,
    pub time: SystemTime,
    pub meta: WorldMeta,
    // TODO: also save view params like current zoom, etc. ptbly through struct like GameView
    pub units_data: Vec<String>,
    // pub chunks_data: Vec<String>,
}

#[derive(Debug)]
pub enum CreateFileError {
    SystemError(String),
    FileExists,
}

impl SaveFile {
    pub fn new(name: &str, seed: &str, size: GalaxySize, class: GalaxyClass) -> Self {
        let name = name
            .trim()
            .replace("\n", "")
            .replace("/", "")
            .replace("\\", "");
        let file_name = name.replace(" ", "_");
        let path: PathBuf = ["save", (file_name + ".save").as_str()].iter().collect();
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let seed = hasher.finish();
        SaveFile {
            path,
            version: VERSION.to_string(),
            time: SystemTime::now(),
            meta: WorldMeta {
                name,
                seed,
                size,
                class,
                current_tick: 0,
            },
            units_data: Vec::new(),
            // chunks_data: Vec::new(),
        }
    }

    pub fn load(path: PathBuf) -> Option<Self> {
        let file = File::open(&path).ok()?;
        let mut lines = BufReader::new(&file).lines();
        let meta = lines.next()?.ok()?;
        if meta.is_empty() {
            return None;
        }
        let meta = serde_json::from_str(meta.as_str()).ok()?;
        let version = lines.next()?.ok()?;
        if version.is_empty() {
            return None;
        }
        let time = lines.next()?.ok()?.parse::<u64>().ok()?;
        let time = SystemTime::UNIX_EPOCH + Duration::new(time, 0);
        let mut units_data = Vec::new();
        loop {
            let unit = lines.next()?.ok()?;
            if unit.eq("/units") {
                break;
            }
            units_data.push(unit);
        }
        // let mut chunks_data = Vec::new();
        // loop {
        //     let chunk = lines.next()?.ok()?;
        //     if chunk.eq("/chunks") {
        //         break;
        //     }
        //     chunks_data.push(chunk);
        // }

        Some(SaveFile {
            path,
            version,
            time,
            meta,
            units_data,
            // chunks_data,
        })
    }

    pub fn create(&mut self) -> Result<(), CreateFileError> {
        create(&self.path, &self.meta)
    }

    pub fn load_avatar(&self) -> Avatar {
        serde_json::from_str(self.units_data.get(0).unwrap().as_str()).unwrap()
    }
}

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
            if let Some(s) = SaveFile::load(p.unwrap().path()) {
                files.push(s);
            }
        }
    }
    files.sort_by(|s1, s2| s2.time.cmp(&s1.time));
    files
}

pub fn delete(path: &Path) {
    if path.exists() {
        remove_file(path).ok();
    }
}

pub fn create(path: &Path, meta: &WorldMeta) -> Result<(), CreateFileError> {
    let dir = Path::new("save");
    if !dir.exists() {
        create_dir(dir).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
    }
    if path.is_file() {
        Err(CreateFileError::FileExists)
    } else {
        let time = SystemTime::now();
        let mut file =
            File::create(&path).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        let data = format!(
            "{}\n{}\n{}\n/units\n/chunks",
            serde_json::to_string(meta).unwrap(),
            VERSION,
            time.duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| CreateFileError::SystemError(e.to_string()))?
                .as_secs(),
        );
        file.write_all(data.as_bytes())
            .map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        Ok(())
    }
}

pub fn save(path: &Path, world: &mut World) -> Result<(), String> {
    let dir = Path::new("save");
    if !dir.exists() {
        create_dir(dir).map_err(|e| e.to_string())?;
    }
    let time = SystemTime::now();
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    let mut data = format!(
        "{}\n{}\n{}\n{}",
        serde_json::to_string(&world.meta).map_err(|e| e.to_string())?,
        VERSION,
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs(),
        serde_json::to_string(&world.avatar).map_err(|e| e.to_string())?
    );
    data.push_str("\n/units");
    // for coords in world.changed.clone().iter() {
    //     let chunk = world.load_chunk(*coords);
    //     data.push('\n');
    //     data.push_str(
    //         serde_json::to_string(chunk)
    //             .map_err(|e| e.to_string())?
    //             .as_str(),
    //     );
    // }
    // data.push_str("\n/chunks");
    file.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}
