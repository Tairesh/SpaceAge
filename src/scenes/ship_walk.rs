use crate::assets::{Assets, TileSet};
use crate::colors::Colors;
use crate::game::action::{Action, ActionType};
use crate::game::ship_parts::ShipPartAction;
use crate::game::ship_tile::ShipTile;
use crate::game::world::World;
use crate::geometry::direction::{Direction, DIR9};
use crate::scenes::{GameScene, Scene, Transition};
use crate::sprites::image::Image;
use crate::sprites::label::Label;
use crate::sprites::position::Position;
use crate::sprites::ship::ShipView;
use crate::sprites::sprite::{Positionate, Sprite};
use crate::{input, Vec2};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier};
use tetra::Context;

#[derive(Debug)]
enum GameMode {
    Default,
    Opening,
    Closing,
}

impl GameMode {
    pub fn draw_cursors(&self) -> bool {
        match self {
            GameMode::Default => false,
            GameMode::Opening | GameMode::Closing => true,
        }
    }

    pub fn cursor_here(&self, tile: &ShipTile) -> bool {
        match self {
            GameMode::Default => false,
            GameMode::Opening => tile.supports_action(ShipPartAction::Open),
            GameMode::Closing => tile.supports_action(ShipPartAction::Close),
        }
    }
}

pub struct ShipWalk {
    #[allow(dead_code)]
    world: Rc<RefCell<World>>,
    assets: Rc<Assets>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    ship_view: Rc<RefCell<ShipView>>,
    last_walk: Instant,
    mode: GameMode,
    cursor: Mesh,
    selected: Option<Direction>,
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
            mode: GameMode::Default,
            cursor: Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(1.0),
                Rectangle::new(
                    0.0,
                    0.0,
                    TileSet::TILE_SIZE.0 as f32,
                    TileSet::TILE_SIZE.1 as f32,
                ),
            )
            .unwrap(),
            selected: None,
        }
    }

    fn select(&mut self, dir: Direction) {
        if self.selected.is_none() {
            self.selected = Some(dir);
            self.world.borrow_mut().avatar.vision = dir;
        }
    }
}

impl Scene for ShipWalk {
    // TODO: positionate ship relative to avatar position
    // TODO: add zoom
    fn update(&mut self, ctx: &mut Context, focused: bool) -> Transition {
        if focused {
            return Transition::DoNothing;
        }

        match self.mode {
            GameMode::Default => {
                if input::is_pressed_key_with_mod(ctx, Key::Escape, None) {
                    return Transition::Push(GameScene::GameMenu);
                } else if input::is_pressed_key_with_mod(ctx, Key::O, None) {
                    self.mode = GameMode::Opening;
                } else if input::is_pressed_key_with_mod(ctx, Key::C, None) {
                    self.mode = GameMode::Closing;
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
            }
            GameMode::Opening => {
                if input::is_pressed_key_with_mod(ctx, Key::Escape, None) {
                    self.mode = GameMode::Default;
                }
                if let Some(dir) = input::get_direction_keys_down(ctx) {
                    if self.selected.is_none() {
                        self.select(dir);
                        let mut world = self.world.borrow_mut();
                        world.avatar.action = Action::new(
                            ActionType::ActivatingPart(dir, ShipPartAction::Open),
                            &world,
                        );
                    }
                } else if self.selected.is_some() {
                    self.mode = GameMode::Default;
                    self.selected = None;
                }
            }
            GameMode::Closing => {
                if input::is_pressed_key_with_mod(ctx, Key::Escape, None) {
                    self.mode = GameMode::Default;
                }
                if let Some(dir) = input::get_direction_keys_down(ctx) {
                    if self.selected.is_none() {
                        self.select(dir);
                        let mut world = self.world.borrow_mut();
                        world.avatar.action = Action::new(
                            ActionType::ActivatingPart(dir, ShipPartAction::Close),
                            &world,
                        );
                    }
                } else if self.selected.is_some() {
                    self.mode = GameMode::Default;
                    self.selected = None;
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

    fn draw(&mut self, ctx: &mut Context) {
        if self.mode.draw_cursors() {
            let world = self.world.borrow();
            let rect = self.ship_view.borrow().rect();
            let center = Vec2::from(world.avatar.pos * TileSet::TILE_SIZE * 2.0) + (rect.x, rect.y);

            for dir in DIR9 {
                let pos = world.avatar.pos + dir;
                if let Some(tile) = world.ship.get_tile(pos) {
                    if self.mode.cursor_here(tile) {
                        let delta = Vec2::new(
                            (dir.dx() * TileSet::TILE_SIZE.0) as f32,
                            (dir.dy() * TileSet::TILE_SIZE.1) as f32,
                        ) * 2.0;
                        self.cursor.draw(
                            ctx,
                            DrawParams::new()
                                .position(center + delta)
                                .scale(Vec2::new(2.0, 2.0))
                                .color(Colors::ORANGE.with_alpha(0.7)),
                        );
                    }
                }
            }

            if let Some(dir) = self.selected {
                let delta = Vec2::new(
                    (dir.dx() * TileSet::TILE_SIZE.0) as f32,
                    (dir.dy() * TileSet::TILE_SIZE.1) as f32,
                ) * 2.0;
                self.cursor.draw(
                    ctx,
                    DrawParams::new()
                        .scale(Vec2::new(2.0, 2.0))
                        .position(center + delta)
                        .color(Colors::LIGHT_YELLOW.with_alpha(0.7)),
                )
            }
        }
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
