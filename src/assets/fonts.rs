use tetra::{graphics::text::Font, Context, Result};

use super::PreparedFont;

#[derive(Debug)]
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
        let consolab = include_bytes!("../../inc/fonts/consolab.ttf");
        let handel = include_bytes!("../../inc/fonts/HandelGothic.ttf");
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
