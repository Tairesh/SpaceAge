use tetra::{
    graphics::ImageData,
    window::{self, WindowPosition::Centered},
    Context, ContextBuilder,
};

use crate::settings::Settings;

pub fn create_context<S: Into<String>>(title: S) -> tetra::Result<Context> {
    let settings = Settings::instance();
    let mut ctx_builder = ContextBuilder::new(title, settings.window.width, settings.window.height);
    ctx_builder
        .show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true);
    let mut ctx = ctx_builder.build()?;

    let mut icon = ImageData::from_encoded(include_bytes!("../inc/img/icon.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;
    window::set_minimum_size(&mut ctx, 1024, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;
    if settings.window.fullscreen {
        window::set_fullscreen(&mut ctx, true).ok();
    } else {
        let monitor = window::get_current_monitor(&ctx).unwrap_or(0);
        window::set_position(&mut ctx, Centered(monitor), Centered(monitor));
    }

    Ok(ctx)
}
