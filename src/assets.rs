use tetra::graphics::text::Font;
use tetra::graphics::Texture;
use tetra::Context;

pub struct Fonts {
    pub consolab: Font,
    pub astrolab: Font,
    pub nasa: Font,
    pub logo: Font,
}

impl Fonts {
    pub fn new(ctx: &mut Context) -> Self {
        let consolab = include_bytes!("../inc/fonts/consolab.ttf");
        let astrolab = include_bytes!("../inc/fonts/astrolab.ttf");
        let nasa = include_bytes!("../inc/fonts/nasalization.ttf");
        Self {
            consolab: Font::from_vector_file_data(ctx, consolab, 18.0).unwrap(),
            astrolab: Font::from_vector_file_data(ctx, astrolab, 32.0).unwrap(),
            nasa: Font::from_vector_file_data(ctx, nasa, 24.0).unwrap(),
            logo: Font::from_vector_file_data(ctx, nasa, 72.0).unwrap(),
        }
    }
}

pub struct Images {
    pub bg: Texture,
}

impl Images {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            bg: Texture::from_file_data(ctx, include_bytes!("../inc/img/bg.jpg")).unwrap(),
        }
    }
}

pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            fonts: Fonts::new(ctx),
            images: Images::new(ctx),
        }
    }
}
