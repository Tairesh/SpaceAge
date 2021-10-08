use crate::colors::Colors;
use crate::scenes::Transition;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Draw, Positionate, Sprite, Update};
use crate::{Rect, Vec2};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Canvas, DrawParams, Rectangle};
use tetra::input::{Key, MouseButton};
use tetra::{graphics, input, window, Context};

fn draw_galaxy(
    ctx: &mut Context,
    size: usize,
    sectors: &[u32],
    name: &str,
    font_title: Font,
    font_bottom: Font,
    zoom: i32,
) -> Canvas {
    let pixel_size = size as i32 * zoom;
    let canvas_size = i32::max(pixel_size, 512);
    let offset = if pixel_size < 512 {
        ((512 - pixel_size) as f32 / 2.0).round()
    } else {
        0.0
    };
    let canvas = Canvas::new(ctx, canvas_size, canvas_size).unwrap();
    graphics::set_canvas(ctx, &canvas);
    let mesh = Mesh::rounded_rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(0.0, 0.0, canvas_size as f32, canvas_size as f32),
        BorderRadii::new(5.0),
    )
    .unwrap();
    mesh.draw(ctx, DrawParams::new().color(Colors::SPACE_VIOLET));

    // draw so many meshes is very slow
    // TODO: use some textures instead, they are batched
    let mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 5.0, 5.0)).unwrap();
    let max = 262_144.0f32;
    for x in 0..size {
        for y in 0..size {
            let i = x * size + y;
            let d = sectors[i] as f32 / max;
            mesh.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        offset + x as f32 * zoom as f32,
                        offset + y as f32 * zoom as f32,
                    ))
                    .color(Colors::LIGHT_YELLOW.with_alpha(d)),
            );
        }
    }
    let mesh = Mesh::rounded_rectangle(
        ctx,
        ShapeStyle::Stroke(2.0),
        Rectangle::new(0.0, 0.0, canvas_size as f32, canvas_size as f32),
        BorderRadii::new(5.0),
    )
    .unwrap();
    mesh.draw(ctx, DrawParams::new().color(Colors::ORANGE));

    let mut text = Text::new(format!("{} galaxy", name), font_title);
    let bounds = text.get_bounds(ctx).unwrap();
    text.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(
                (canvas_size as f32) / 2.0 - bounds.width / 2.0,
                10.0,
            ))
            .color(Colors::ORANGE),
    );
    let sum: u64 = sectors.iter().copied().map(u64::from).sum();
    let mut text = Text::new(
        format!("{} stars in {}x{} sectors", sum, size, size),
        font_bottom,
    );
    let bounds = text.get_bounds(ctx).unwrap();
    text.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(
                (canvas_size as f32) / 2.0 - bounds.width / 2.0,
                canvas_size as f32 - bounds.height - 10.0,
            ))
            .color(Colors::LIGHT_GRAY),
    );

    graphics::reset_canvas(ctx);
    canvas
}

pub struct Galaxy {
    canvas: Option<Canvas>,
    font_title: Font,
    font_bottom: Font,
    size: usize,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Galaxy {
    pub fn new(size: usize, font_title: Font, font_bottom: Font, position: Position) -> Self {
        Self {
            canvas: None,
            font_title,
            font_bottom,
            size,
            position,
            rect: None,
            visible: false,
        }
    }

    pub fn redraw(&mut self, ctx: &mut Context, size: usize, sectors: Vec<u32>, name: &str) {
        self.size = size;
        self.canvas = Some(draw_galaxy(
            ctx,
            size,
            &sectors,
            name,
            self.font_title.clone(),
            self.font_bottom.clone(),
            3,
        ));
        self.positionate(ctx, window::get_size(ctx));
        self.set_focused(true)
    }
}

impl Draw for Galaxy {
    fn draw(&mut self, ctx: &mut Context) {
        if let Some(canvas) = &self.canvas {
            let rect = self.rect.unwrap();
            canvas.draw(ctx, Vec2::new(rect.x, rect.y));
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Galaxy {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        if let Some(canvas) = &self.canvas {
            let (x, y) = canvas.size();
            Vec2::new(x as f32, y as f32)
        } else {
            Vec2::zero()
        }
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect)
    }
}

impl Update for Galaxy {
    fn update(
        &mut self,
        ctx: &mut Context,
        _focused: bool,
        _blocked: &[Rect],
    ) -> Option<Transition> {
        if self.focused() {
            if input::is_key_pressed(ctx, Key::Escape)
                || input::is_mouse_button_down(ctx, MouseButton::X1)
            {
                self.set_focused(false);
            }
            if input::is_mouse_button_down(ctx, MouseButton::Left) {
                let mouse = input::get_mouse_position(ctx);
                if !self.rect.unwrap().contains_point(mouse) {
                    self.set_focused(false)
                }
            }
        }
        None
    }
}

impl Sprite for Galaxy {
    fn focused(&self) -> bool {
        self.visible()
    }

    fn set_focused(&mut self, focused: bool) {
        self.set_visible(focused)
    }
}
