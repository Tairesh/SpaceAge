use crate::assets::TileSet;
use crate::avatar::Avatar;
use crate::geometry::point::Point;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Draw, Positionate, Sprite, Update};
use crate::things::ship::Ship;
use crate::{Rect, Vec2};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{Canvas, DrawParams, Rectangle};
use tetra::{graphics, Context};

fn draw_ship(ctx: &mut Context, ship: &Ship, avatar: &Avatar, tileset: &TileSet) -> Canvas {
    let canvas_size = (
        TileSet::TILE_SIZE.0 * ship.bounds.0 as i32,
        TileSet::TILE_SIZE.1 * ship.bounds.1 as i32,
    );
    let canvas = Canvas::new(ctx, canvas_size.0, canvas_size.1).unwrap();
    graphics::set_canvas(ctx, &canvas);
    let mesh = Mesh::rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(
            0.0,
            0.0,
            TileSet::TILE_SIZE.0 as f32,
            TileSet::TILE_SIZE.1 as f32,
        ),
    )
    .unwrap();
    for (i, tile) in ship.tiles.iter().enumerate() {
        if tile.is_void() {
            continue;
        }
        let point = Point::from_index(i, ship.bounds.0);
        let pos = Vec2::from(point * TileSet::TILE_SIZE);
        if let Some(color) = tile.bg_color() {
            mesh.draw(ctx, DrawParams::new().position(pos).color(color));
        }
        tileset.draw(
            ctx,
            tile.ch(),
            DrawParams::new().position(pos).color(tile.color()),
        );
        if avatar.pos == point {
            tileset.draw(
                ctx,
                '@',
                DrawParams::new()
                    .position(pos)
                    .color(avatar.character.skin_tone.into()),
            );
        }
    }
    graphics::reset_canvas(ctx);
    canvas
}

pub struct ShipView {
    canvas: Canvas,
    position: Position,
    rect: Option<Rect>,
    zoom: f32,
    visible: bool,
}

impl ShipView {
    pub fn new(
        ctx: &mut Context,
        ship: &Ship,
        avatar: &Avatar,
        tileset: &TileSet,
        position: Position,
    ) -> Self {
        Self {
            canvas: draw_ship(ctx, ship, avatar, tileset),
            position,
            rect: None,
            zoom: 2.0,
            visible: true,
        }
    }
}

impl Draw for ShipView {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.canvas.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .scale(Vec2::new(self.zoom, self.zoom)),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for ShipView {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        let size = self.canvas.size();
        Vec2::new(size.0 as f32 * self.zoom, size.1 as f32 * self.zoom)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for ShipView {}
impl Sprite for ShipView {}
