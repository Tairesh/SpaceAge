use super::galaxy_class::GalaxyClass;
use super::galaxy_size::GalaxySize;
use crate::astro::galaxy_generator::generate_quadrants;
use crate::astro::quadrant::Quadrant;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalaxyMeta {
    pub name: String,
    pub seed: u64,
    pub size: GalaxySize,
    pub class: GalaxyClass,
}

impl GalaxyMeta {
    pub fn new(name: String, seed: String, size: GalaxySize, class: GalaxyClass) -> Self {
        let name = name
            .trim()
            .replace('\n', "")
            .replace('/', "")
            .replace('\\', "");
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let seed = hasher.finish();
        Self {
            name,
            seed,
            size,
            class,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Galaxy {
    pub meta: GalaxyMeta,
    pub quadrants: Vec<Quadrant>,
}

impl From<GalaxyMeta> for Galaxy {
    fn from(meta: GalaxyMeta) -> Self {
        Self {
            quadrants: generate_quadrants(meta.seed, meta.size.into(), meta.class)
                .into_iter()
                .map(Quadrant::new)
                .collect(),
            meta,
        }
    }
}
