use std::rc::Rc;

use tetra::Context;

pub use self::{fonts::Fonts, images::Images, prepared_font::PreparedFont, tileset::TileSet};

mod fonts;
mod images;
mod prepared_font;
mod tileset;

// Can't put this to OnceCell because tetra::Font and tetra::Texture uses Rc<> inside
pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
    pub tileset: Rc<TileSet>,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            fonts: Fonts::new(ctx)?,
            images: Images::new(ctx)?,
            tileset: Rc::new(TileSet::new(ctx)?),
        })
    }
}
