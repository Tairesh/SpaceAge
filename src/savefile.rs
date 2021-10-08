use crate::astro::galaxy_generator;
use crate::world::WorldMeta;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveFileMeta {
    pub version: String,
    pub time: SystemTime,
    world_meta: WorldMeta,
}

impl SaveFileMeta {
    pub fn new(world_meta: WorldMeta) -> Self {
        Self {
            version: VERSION.to_string(),
            time: SystemTime::now(),
            world_meta,
        }
    }

    pub fn create(&mut self) -> Result<(), CreateFileError> {
        let dir = Path::new("save");
        if !dir.exists() {
            std::fs::create_dir(dir).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        }
        let file_name = self.world_meta.name.replace(" ", "_");
        let path: PathBuf = ["save", (file_name + ".save").as_str()].iter().collect();
        if path.is_file() {
            return Err(CreateFileError::FileExists);
        }
        self.time = SystemTime::now();
        let mut file =
            File::create(&path).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
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

    pub fn load(path: &Path) -> Option<Self> {
        let file = File::open(path).ok()?;
        let mut lines = BufReader::new(&file).lines();
        let meta = lines.next()?.ok()?;
        serde_json::from_str(meta.as_str()).ok()
    }

    pub fn name(&self) -> &str {
        self.world_meta.name.as_str()
    }
}

pub fn savefiles() -> Vec<(PathBuf, SaveFileMeta)> {
    let path = Path::new("save");
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            let p = p.unwrap().path();
            if let Some(s) = SaveFileMeta::load(&p) {
                files.push((p, s));
            }
        }
    }
    files.sort_by(|(_, s1), (_, s2)| s2.time.cmp(&s1.time));
    files
}
