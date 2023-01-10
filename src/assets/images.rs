use tetra::{graphics::Texture, Context, Result};

#[derive(Debug)]
pub struct Images {
    pub eclipse: Texture,
    pub icon: Texture,
    pub blue_nebula: Texture,
}

impl Images {
    pub fn new(ctx: &mut Context) -> Result<Self> {
        Ok(Self {
            eclipse: Texture::from_encoded(ctx, include_bytes!("../../inc/img/eclipse.jpg"))?,
            icon: Texture::from_encoded(ctx, include_bytes!("../../inc/img/icon.png"))?,
            blue_nebula: Texture::from_encoded(
                ctx,
                include_bytes!("../../inc/img/blue_nebula.jpg"),
            )?,
        })
    }
}
