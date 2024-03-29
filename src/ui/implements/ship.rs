use crate::ascii::tile::Tile;
use crate::assets::TileSet;
use crate::colors::Colors;
use crate::fov::field_of_view_set;
use crate::game::avatar::Avatar;
use crate::game::ship::Ship;
use crate::ui::{Draw, Focus, Horizontal, Position, Positionate, UiSprite, Update, Vertical};
use geometry::{Point, Rect, Vec2};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::graphics::{Canvas, DrawParams, Rectangle};
use tetra::{graphics, window, Context};

fn draw_ship(ctx: &mut Context, ship: &Ship, avatar: &Avatar, tileset: &TileSet) -> Canvas {
    let canvas_size = (
        TileSet::TILE_SIZE.0 * ship.bounds.0,
        TileSet::TILE_SIZE.1 * ship.bounds.1,
    );
    let canvas = Canvas::new(ctx, canvas_size.0, canvas_size.1).unwrap();
    graphics::set_canvas(ctx, &canvas);

    let mut bg_builder = GeometryBuilder::new();
    let bg_color = Colors::SPACE_VIOLET;
    for (i, tile) in ship.tiles.iter().enumerate() {
        if tile.is_void() {
            continue;
        }
        let tile: Tile = tile.into();
        if let Some(bg) = tile.bg {
            if bg == bg_color {
                let point = Point::from_index(i, ship.bounds.0);
                let pos = Vec2::from(point * TileSet::TILE_SIZE);
                bg_builder
                    .rectangle(
                        ShapeStyle::Fill,
                        Rectangle::new(
                            pos.x,
                            pos.y,
                            TileSet::TILE_SIZE.0 as f32,
                            TileSet::TILE_SIZE.1 as f32,
                        ),
                    )
                    .ok();
            }
        }
    }
    let mesh = bg_builder.build_mesh(ctx).unwrap();
    mesh.draw(ctx, DrawParams::new().color(bg_color));
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

    let fov = field_of_view_set(avatar.pos, ship.bounds.0.max(ship.bounds.1) as u32, ship);
    let mut fov_builder = GeometryBuilder::new();
    for (i, tile) in ship.tiles.iter().enumerate() {
        if tile.is_void() {
            continue;
        }
        let tile: Tile = tile.into();
        let point = Point::from_index(i, ship.bounds.0);
        let pos = Vec2::from(point * TileSet::TILE_SIZE);
        if let Some(color) = tile.bg {
            if !fov.contains(&point) {
                fov_builder
                    .rectangle(
                        ShapeStyle::Fill,
                        Rectangle::new(
                            pos.x,
                            pos.y,
                            TileSet::TILE_SIZE.0 as f32,
                            TileSet::TILE_SIZE.1 as f32,
                        ),
                    )
                    .ok();
            }
            if color != bg_color {
                mesh.draw(ctx, DrawParams::new().position(pos).color(color));
            }
        }
        if avatar.pos == point {
            tileset.draw(
                ctx,
                '@',
                DrawParams::new()
                    .position(pos)
                    .color(avatar.character.skin_tone.into()),
            );
        } else {
            tileset.draw(ctx, tile.ch, DrawParams::new().position(pos).color(tile.fg));
        }
    }

    let mesh = fov_builder.build_mesh(ctx).unwrap();
    mesh.draw(ctx, DrawParams::new().color(Colors::BLACK.with_alpha(0.5)));

    graphics::reset_canvas(ctx);
    canvas
}

pub struct ShipView {
    canvas: Canvas,
    position: Position,
    rect: Option<Rect>,
    zoom: f32,
    visible: bool,
    avatar_pos: Point,
}

fn position(avatar_pos: Point, zoom: f32) -> Position {
    let pos = avatar_pos * TileSet::TILE_SIZE * zoom + TileSet::TILE_SIZE;
    Position {
        x: Horizontal::AtWindowCenterByLeft {
            offset: -pos.x as f32,
        },
        y: Vertical::AtWindowCenterByTop {
            offset: -pos.y as f32,
        },
    }
}

impl ShipView {
    pub fn new(
        ctx: &mut Context,
        ship: &Ship,
        avatar: &Avatar,
        tileset: &TileSet,
        zoom: f32,
    ) -> Self {
        Self {
            canvas: draw_ship(ctx, ship, avatar, tileset),
            position: position(avatar.pos, zoom),
            rect: None,
            zoom,
            visible: true,
            avatar_pos: avatar.pos,
        }
    }

    fn repositionate(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        self.position = position(self.avatar_pos, self.zoom);
        self.positionate(ctx, window_size);
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        ship: &Ship,
        avatar: &Avatar,
        tileset: &TileSet,
        window_size: (i32, i32),
    ) {
        self.canvas = draw_ship(ctx, ship, avatar, tileset);
        self.avatar_pos = avatar.pos;
        self.repositionate(ctx, window_size);
    }

    pub fn set_zoom(&mut self, zoom: f32, ctx: &mut Context) {
        self.zoom = zoom;
        self.repositionate(ctx, window::get_size(ctx));
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
        let (width, height) = self.canvas.size();
        Vec2::new(width as f32, height as f32) * self.zoom
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for ShipView {}

impl Focus for ShipView {}

impl UiSprite for ShipView {}
