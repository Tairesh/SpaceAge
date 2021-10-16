use crate::assets::Assets;
use crate::colors::Colors;
use crate::scenes::{GameScene, Scene, Transition};
use crate::sprites::image::Image;
use crate::sprites::label::Label;
use crate::sprites::position::Position;
use crate::sprites::ship::ShipView;
use crate::sprites::sprite::Sprite;
use crate::things::world::World;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::{Context, Event};

pub struct ShipWalk {
    #[allow(dead_code)]
    world: Rc<RefCell<World>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl ShipWalk {
    pub fn new(world: Rc<RefCell<World>>, assets: &Assets, ctx: &mut Context) -> Self {
        let bg = Rc::new(RefCell::new(Image::repeat(
            assets.images.blue_nebula.clone(),
        )));
        let name = Rc::new(RefCell::new(Label::new(
            world.borrow().ship.name.clone(),
            assets.fonts.astrolab16.clone(),
            Colors::LIGHT_SKY_BLUE,
            Position::by_left_top(10.0, 10.0),
        )));
        let ship = Rc::new(RefCell::new(ShipView::new(
            ctx,
            &world.borrow().ship,
            &assets.tileset,
            Position::center(),
        )));
        Self {
            world,
            sprites: vec![bg, name, ship],
        }
    }
}

impl Scene for ShipWalk {
    fn event(&mut self, _ctx: &mut Context, event: Event, _focused: bool) -> Transition {
        if let Event::KeyPressed { key: Key::Escape } = event {
            Transition::Push(GameScene::GameMenu)
        } else {
            Transition::DoNothing
        }
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
