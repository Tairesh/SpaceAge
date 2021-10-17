use crate::ascii::cp437::char_to_point;
use tetra::graphics::text::Font;
use tetra::graphics::{DrawParams, Rectangle, Texture};
use tetra::Context;

pub struct Fonts {
    pub consolab18: Font,
    pub consolab12: Font,
    pub astrolab32: Font,
    pub astrolab16: Font,
    pub nasa24: Font,
    pub nasa14: Font,
    pub logo: Font,
}

impl Fonts {
    pub fn new(ctx: &mut Context) -> Self {
        let consolab = include_bytes!("../inc/fonts/consolab.ttf");
        let astrolab = include_bytes!("../inc/fonts/astrolab.ttf");
        let nasa = include_bytes!("../inc/fonts/nasalization.ttf");
        Self {
            consolab18: Font::from_vector_file_data(ctx, consolab, 18.0).unwrap(),
            consolab12: Font::from_vector_file_data(ctx, consolab, 12.0).unwrap(),
            astrolab32: Font::from_vector_file_data(ctx, astrolab, 32.0).unwrap(),
            astrolab16: Font::from_vector_file_data(ctx, astrolab, 16.0).unwrap(),
            nasa24: Font::from_vector_file_data(ctx, nasa, 24.0).unwrap(),
            nasa14: Font::from_vector_file_data(ctx, nasa, 14.0).unwrap(),
            logo: Font::from_vector_file_data(ctx, nasa, 72.0).unwrap(),
        }
    }
}

pub struct Images {
    pub bg: Texture,
    pub icon: Texture,
    pub blue_nebula: Texture,
}

impl Images {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            bg: Texture::from_file_data(ctx, include_bytes!("../inc/img/bg.jpg")).unwrap(),
            icon: Texture::from_file_data(ctx, include_bytes!("../inc/img/icon.png")).unwrap(),
            blue_nebula: Texture::from_file_data(ctx, include_bytes!("../inc/img/blue_nebula.jpg"))
                .unwrap(),
        }
    }
}

pub struct TileSet {
    pub texture: Texture,
}

impl TileSet {
    pub const TILE_SIZE: (i32, i32) = (12, 12);

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            texture: Texture::from_file_data(ctx, include_bytes!("../inc/img/12x12.png")).unwrap(),
        }
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
    pub fn load(ctx: &mut Context) -> Self {
        Self {
            fonts: Fonts::new(ctx),
            images: Images::new(ctx),
            tileset: TileSet::new(ctx),
        }
    }
}
