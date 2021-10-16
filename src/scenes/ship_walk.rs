use crate::assets::Assets;
use crate::colors::Colors;
use crate::game::action::{Action, ActionType};
use crate::game::world::World;
use crate::input;
use crate::scenes::{GameScene, Scene, Transition};
use crate::sprites::image::Image;
use crate::sprites::label::Label;
use crate::sprites::position::Position;
use crate::sprites::ship::ShipView;
use crate::sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

pub struct ShipWalk {
    #[allow(dead_code)]
    world: Rc<RefCell<World>>,
    assets: Rc<Assets>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    ship_view: Rc<RefCell<ShipView>>,
    last_walk: Instant,
}

impl ShipWalk {
    pub fn new(world: Rc<RefCell<World>>, assets: Rc<Assets>, ctx: &mut Context) -> Self {
        let bg = Rc::new(RefCell::new(Image::repeat(
            assets.images.blue_nebula.clone(),
        )));
        let name = Rc::new(RefCell::new(Label::new(
            world.borrow().ship.name.clone(),
            assets.fonts.astrolab16.clone(),
            Colors::LIGHT_SKY_BLUE,
            Position::by_left_top(10.0, 10.0),
        )));
        let ship_view = Rc::new(RefCell::new(ShipView::new(
            ctx,
            &world.borrow().ship,
            &world.borrow().avatar,
            &assets.tileset,
            Position::center(),
        )));
        Self {
            world,
            assets,
            sprites: vec![bg, name, ship_view.clone()],
            ship_view,
            last_walk: Instant::now(),
        }
    }
}

impl Scene for ShipWalk {
    fn update(&mut self, ctx: &mut Context, focused: bool) -> Transition {
        if focused {
            return Transition::DoNothing;
        }

        let now = Instant::now();
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if now.duration_since(self.last_walk).as_millis() > 75
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                let mut world = self.world.borrow_mut();
                if dir.is_here() {
                    world.avatar.action = Action::new(ActionType::SkippingTime, &world);
                } else {
                    world.avatar.action = Action::new(ActionType::Walking(dir), &world);
                }
            }
        }

        if self.world.borrow().avatar.action.is_some() {
            let mut world = self.world.borrow_mut();
            world.tick();
            self.ship_view.borrow_mut().update(
                ctx,
                &world.ship,
                &world.avatar,
                &self.assets.tileset,
            );
        }

        Transition::DoNothing
    }

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
