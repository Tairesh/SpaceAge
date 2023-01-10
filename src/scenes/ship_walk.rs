use crate::assets::{Assets, TileSet};
use crate::colors::Colors;
use crate::game::action::{Action, ActionType};
use crate::game::ship_parts::ShipPartAction;
use crate::game::ship_tile::ShipTile;
use crate::game::world::World;
use crate::input;
use crate::scenes::{GameScene, Scene, Transition};
use crate::ui::{Image, Label, Position, Positionate, ShipView, UiSprite};
use geometry::Vec2;
use geometry::{Direction, DIR9};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier};
use tetra::{window, Context};

#[derive(Debug)]
enum GameMode {
    Default,
    Activating(Option<ShipPartAction>),
}

impl GameMode {
    pub fn draw_cursors(&self) -> bool {
        match self {
            GameMode::Default => false,
            GameMode::Activating(..) => true,
        }
    }

    pub fn cursor_here(&self, tile: &ShipTile) -> bool {
        match self {
            GameMode::Default => false,
            GameMode::Activating(action) => {
                if let Some(action) = action {
                    tile.supports_action(*action)
                } else {
                    tile.supports_any_action()
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Zoom(u8);

impl Zoom {
    pub fn as_view(&self) -> f32 {
        (*self).into()
    }

    pub fn as_scale(&self) -> Vec2 {
        let f = self.as_view();
        Vec2::new(f, f)
    }

    pub fn inc(&mut self) -> bool {
        if self.0 < 7 {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    pub fn dec(&mut self) -> bool {
        if self.0 > 1 {
            self.0 -= 1;
            true
        } else {
            false
        }
    }
}

impl From<Zoom> for f32 {
    fn from(z: Zoom) -> Self {
        match z.0 {
            1 | 0 => 0.25,
            2 => 0.5,
            3 => 1.0,
            4 => 2.0,
            5 => 3.0,
            6 => 4.0,
            7.. => 5.0,
        }
    }
}

pub struct ShipWalk {
    #[allow(dead_code)]
    world: Rc<RefCell<World>>,
    assets: Rc<Assets>,
    sprites: Vec<Rc<RefCell<dyn UiSprite>>>,
    ship_view: Rc<RefCell<ShipView>>,
    clock: Rc<RefCell<Label>>,
    last_walk: Instant,
    mode: GameMode,
    cursor: Mesh,
    selected: Option<Direction>,
    zoom: Zoom,
}

impl ShipWalk {
    pub fn new(world: Rc<RefCell<World>>, assets: Rc<Assets>, ctx: &mut Context) -> Self {
        let bg = Rc::new(RefCell::new(Image::repeat(
            assets.images.blue_nebula.clone(),
        )));
        let name = Rc::new(RefCell::new(Label::new(
            world.borrow().ship.name.clone(),
            assets.fonts.handel16.clone(),
            Colors::LIGHT_SKY_BLUE,
            Position::by_left_top(10.0, 10.0),
        )));
        let zoom = Zoom(4); // TODO: load current zoom from savefile
        let ship_view = Rc::new(RefCell::new(ShipView::new(
            ctx,
            &world.borrow().ship,
            &world.borrow().avatar,
            &assets.tileset,
            zoom.as_view(),
        )));
        // TODO: implement a graphic clock with binary display
        let clock = Rc::new(RefCell::new(Label::new(
            format!("{}", world.borrow().time()),
            assets.fonts.handel16.clone(),
            Colors::ORANGE,
            Position::by_right_top(-10.0, 10.0),
        )));
        Self {
            world,
            assets,
            sprites: vec![bg, name, ship_view.clone(), clock.clone()],
            ship_view,
            clock,
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
            zoom,
        }
    }

    fn select(&mut self, dir: Direction) {
        if self.selected.is_none() {
            self.selected = Some(dir);
            self.world.borrow_mut().avatar.vision = dir;
        }
    }

    fn draw_cursor(&self, ctx: &mut Context, dir: Direction, color: Color) {
        let rect = self.ship_view.borrow().rect();
        let center =
            Vec2::from(self.world.borrow().avatar.pos * TileSet::TILE_SIZE * self.zoom.as_view())
                + (rect.x, rect.y);
        let delta = Vec2::new(
            (dir.dx() * TileSet::TILE_SIZE.0) as f32,
            (dir.dy() * TileSet::TILE_SIZE.1) as f32,
        ) * self.zoom.as_view();
        self.cursor.draw(
            ctx,
            DrawParams::new()
                .position(center + delta)
                .scale(self.zoom.as_scale())
                .color(color),
        );
    }
}

impl Scene for ShipWalk {
    fn update(&mut self, ctx: &mut Context, focused: bool) -> Transition {
        if focused {
            return Transition::DoNothing;
        }

        // TODO: move it away, see Necromanzer code
        match self.mode {
            GameMode::Default => {
                if (input::is_mouse_scrolled_down(ctx) && self.zoom.dec())
                    || (input::is_mouse_scrolled_up(ctx) && self.zoom.inc())
                {
                    self.ship_view
                        .borrow_mut()
                        .set_zoom(self.zoom.as_view(), ctx);
                }
                if input::is_key_with_mod_pressed(ctx, Key::Escape) {
                    return Transition::Push(GameScene::GameMenu);
                } else if input::is_key_with_mod_pressed(ctx, Key::O) {
                    self.mode = GameMode::Activating(Some(ShipPartAction::Open));
                } else if input::is_key_with_mod_pressed(ctx, Key::C) {
                    self.mode = GameMode::Activating(Some(ShipPartAction::Close));
                } else if input::is_key_with_mod_pressed(ctx, Key::A) {
                    self.mode = GameMode::Activating(None);
                }
                // TODO: Key::E to examine, Key::T to talk, Key::I to inventory view, Key::Q to drop an item, etc.

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
            GameMode::Activating(action) => {
                if input::is_key_with_mod_pressed(ctx, Key::Escape) {
                    self.mode = GameMode::Default;
                }
                if let Some(dir) = input::get_direction_keys_down(ctx) {
                    if self.selected.is_none() {
                        self.select(dir);
                        if let Some(action) = action {
                            let mut world = self.world.borrow_mut();
                            world.avatar.action =
                                Action::new(ActionType::ActivatingPart(dir, action), &world);
                        } else {
                            // TODO: select action from list
                            let world = self.world.borrow();
                            if let Some(tile) = world.ship.get_tile(world.avatar.pos + dir) {
                                if tile.supports_action(ShipPartAction::UseTerminal) {
                                    return Transition::Push(GameScene::Terminal);
                                }
                            }
                        }
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
            let window_size = window::get_size(ctx);
            self.clock
                .borrow_mut()
                .update(format!("{}", world.time()), ctx, window_size);
            self.ship_view.borrow_mut().update(
                ctx,
                &world.ship,
                &world.avatar,
                &self.assets.tileset,
                window_size,
            );
        }

        Transition::DoNothing
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.mode.draw_cursors() {
            for dir in DIR9 {
                if matches!(dir, Direction::Here)
                    && matches!(
                        self.mode,
                        GameMode::Activating(Some(ShipPartAction::Open | ShipPartAction::Close))
                    )
                {
                    continue;
                }
                let pos = self.world.borrow().avatar.pos + dir;
                if let Some(tile) = self.world.borrow().ship.get_tile(pos) {
                    if self.mode.cursor_here(tile) {
                        self.draw_cursor(ctx, dir, Colors::ORANGE.with_alpha(0.7));
                    }
                }
            }

            if let Some(dir) = self.selected {
                self.draw_cursor(ctx, dir, Colors::LIGHT_YELLOW.with_alpha(0.7));
            }
        }
    }

    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn UiSprite>>>> {
        Some(&self.sprites)
    }
}
