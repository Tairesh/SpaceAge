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
    pub const TILE_SIZE: i32 = 10;

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            texture: Texture::from_file_data(ctx, include_bytes!("../inc/img/ascii.png")).unwrap(),
        }
    }

    pub fn draw<P: Into<DrawParams>>(&self, ctx: &mut Context, tile: AsciiTile, params: P) {
        self.texture.draw_region(ctx, tile.into(), params);
    }
}

#[derive(Debug)]
pub enum AsciiTile {
    Empty,
    BigM,
    SmallB,
    SmallD,
    SmallH,
    Door,
    Minus,
    Dot,
    Terminal,
    DoubleLeftTop,          // "╔"
    DoubleRightTop,         // "╗"
    DoubleLeftBottom,       // "╚"
    DoubleRightBottom,      // "╝"
    DoubleVertical,         // "║"
    DoubleVerticalRight,    // "╣"
    DoubleVerticalLeft,     // "╠"
    DoubleHorizontal,       // "═"
    DoubleHorizontalTop,    // "╦"
    DoubleHorizontalBottom, // "╩"
    DoubleCross,            // "╬"
}

impl From<char> for AsciiTile {
    fn from(ch: char) -> Self {
        match ch {
            ' ' => AsciiTile::Empty,
            'M' => AsciiTile::BigM,
            'd' => AsciiTile::SmallD,
            'b' => AsciiTile::SmallB,
            'h' => AsciiTile::SmallH,
            '+' => AsciiTile::Door,
            '-' => AsciiTile::Minus,
            '.' => AsciiTile::Dot,
            '@' => AsciiTile::Terminal,
            '╔' => AsciiTile::DoubleLeftTop,
            '╗' => AsciiTile::DoubleRightTop,
            '╚' => AsciiTile::DoubleLeftBottom,
            '╝' => AsciiTile::DoubleRightBottom,
            '║' => AsciiTile::DoubleVertical,
            '╣' => AsciiTile::DoubleVerticalRight,
            '╠' => AsciiTile::DoubleVerticalLeft,
            '═' => AsciiTile::DoubleHorizontal,
            '╦' => AsciiTile::DoubleHorizontalTop,
            '╩' => AsciiTile::DoubleHorizontalBottom,
            '╬' => AsciiTile::DoubleCross,
            _ => unimplemented!("'{}' is not valid ascii tile", ch),
            // _ => AsciiTile::Empty,
        }
    }
}

impl From<AsciiTile> for Rectangle {
    fn from(tile: AsciiTile) -> Self {
        let coords = match tile {
            AsciiTile::Empty => (0, 0),
            AsciiTile::Door => (0, 150),
            AsciiTile::Minus => (130, 20),
            AsciiTile::Dot => (140, 20),
            AsciiTile::Terminal => (80, 140),
            AsciiTile::DoubleVerticalRight => (90, 110),
            AsciiTile::DoubleVertical => (100, 110),
            AsciiTile::DoubleRightTop => (110, 110),
            AsciiTile::DoubleRightBottom => (120, 110),
            AsciiTile::DoubleLeftBottom => (80, 120),
            AsciiTile::DoubleLeftTop => (90, 120),
            AsciiTile::DoubleHorizontalBottom => (100, 120),
            AsciiTile::DoubleHorizontalTop => (110, 120),
            AsciiTile::DoubleVerticalLeft => (120, 120),
            AsciiTile::DoubleHorizontal => (130, 120),
            AsciiTile::DoubleCross => (140, 120),
            AsciiTile::BigM => (130, 40),
            AsciiTile::SmallB => (20, 60),
            AsciiTile::SmallD => (40, 60),
            AsciiTile::SmallH => (80, 60),
        };
        Rectangle::new(
            coords.0 as f32,
            coords.1 as f32,
            TileSet::TILE_SIZE as f32,
            TileSet::TILE_SIZE as f32,
        )
    }
}

pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
    pub tileset: TileSet,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            fonts: Fonts::new(ctx),
            images: Images::new(ctx),
            tileset: TileSet::new(ctx),
        }
    }
}
