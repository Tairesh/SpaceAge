pub use self::implements::{
    bg::Bg,
    button::Button,
    galaxy::Galaxy,
    image::Image,
    inputs::TextInput,
    label::Label,
    meshy::{HoverableMesh, JustMesh},
    ship::ShipView,
};
pub use self::position::{Horizontal, Position, Vertical};
pub use self::traits::{
    Colorize, Disable, Draw, Focus, Hover, Positionate, Press, Stringify, UiSprite, Update,
};

mod implements;
mod position;
mod traits;

// TODO: use this instead of Vec<Rc<RefCell<dyn UiSprite>>>
// pub type SomeUISpritesMut<'a> = Option<&'a mut [Box<dyn UiSprite>]>;
// pub type SomeUISprites<'a> = Option<&'a [Box<dyn UiSprite>]>;
