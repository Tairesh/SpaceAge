use crate::ascii::cp437::char_to_point;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{DrawParams, Rectangle, Texture};
use tetra::{Context, Result};

#[derive(Debug, Clone)]
pub struct PreparedFont {
    pub font: Font,
    pub line_height: f32,
}

impl PreparedFont {
    pub fn new(ctx: &mut Context, font: Font) -> Self {
        let bounds = Text::new("IjqgpT})@", font.clone())
            .get_bounds(ctx)
            .unwrap();
        Self {
            font,
            line_height: bounds.height,
        }
    }
}

pub struct Fonts {
    pub consolab12: PreparedFont,
    pub consolab18: PreparedFont,
    pub handel14: PreparedFont,
    pub handel16: PreparedFont,
    pub handel24: PreparedFont,
    pub handel32: PreparedFont,
    pub logo: PreparedFont,
}

impl Fonts {
    pub fn new(ctx: &mut Context) -> Result<Self> {
        let consolab = include_bytes!("../inc/fonts/consolab.ttf");
        let handel = include_bytes!("../inc/fonts/HandelGothic.ttf");
        let consolab12 = Font::from_vector_file_data(ctx, consolab, 12.0)?;
        let consolab18 = Font::from_vector_file_data(ctx, consolab, 18.0)?;
        let handel14 = Font::from_vector_file_data(ctx, handel, 14.0)?;
        let handel16 = Font::from_vector_file_data(ctx, handel, 16.0)?;
        let handel24 = Font::from_vector_file_data(ctx, handel, 24.0)?;
        let handel32 = Font::from_vector_file_data(ctx, handel, 32.0)?;
        let handel72 = Font::from_vector_file_data(ctx, handel, 72.0)?;
        Ok(Self {
            consolab12: PreparedFont::new(ctx, consolab12),
            consolab18: PreparedFont::new(ctx, consolab18),
            handel14: PreparedFont::new(ctx, handel14),
            handel16: PreparedFont::new(ctx, handel16),
            handel24: PreparedFont::new(ctx, handel24),
            handel32: PreparedFont::new(ctx, handel32),
            logo: PreparedFont::new(ctx, handel72),
        })
    }
}

pub struct Images {
    pub eclipse: Texture,
    pub icon: Texture,
    pub blue_nebula: Texture,
}

impl Images {
    pub fn new(ctx: &mut Context) -> Result<Self> {
        Ok(Self {
            eclipse: Texture::from_file_data(ctx, include_bytes!("../inc/img/eclipse.jpg"))?,
            icon: Texture::from_file_data(ctx, include_bytes!("../inc/img/icon.png"))?,
            blue_nebula: Texture::from_file_data(
                ctx,
                include_bytes!("../inc/img/blue_nebula.jpg"),
            )?,
        })
    }
}

pub struct TileSet {
    pub texture: Texture,
}

impl TileSet {
    pub const TILE_SIZE: (i32, i32) = (12, 12);

    pub fn new(ctx: &mut Context) -> Result<Self> {
        Ok(Self {
            texture: Texture::from_file_data(ctx, include_bytes!("../inc/img/12x12.png"))?,
        })
    }

    pub fn draw<P: Into<DrawParams>>(&self, ctx: &mut Context, ch: char, params: P) {
        self.texture
            .draw_region(ctx, TileSet::char_to_rectangle(ch), params);
    }

    fn char_to_rectangle(ch: char) -> Rectangle {
        let point = char_to_point(ch);
        Rectangle::new(
            point.x as f32 * TileSet::TILE_SIZE.0 as f32,
            point.y as f32 * TileSet::TILE_SIZE.1 as f32,
            TileSet::TILE_SIZE.0 as f32,
            TileSet::TILE_SIZE.1 as f32,
        )
    }
}

pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
    pub tileset: TileSet,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> Result<Self> {
        Ok(Self {
            fonts: Fonts::new(ctx)?,
            images: Images::new(ctx)?,
            tileset: TileSet::new(ctx)?,
        })
    }
}
