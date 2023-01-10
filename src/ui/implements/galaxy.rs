use crate::assets::PreparedFont;
use crate::colors::Colors;
use crate::scenes::Transition;
use crate::ui::{Draw, Focus, Position, Positionate, UiSprite, Update};
use geometry::{Rect, Vec2};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{Canvas, DrawParams, Rectangle, Texture, TextureFormat};
use tetra::input::{Key, MouseButton};
use tetra::{graphics, input, window, Context};

fn draw_galaxy(
    ctx: &mut Context,
    size: usize,
    quadrants: &[u32],
    name: &str,
    font_title: PreparedFont,
    font_bottom: PreparedFont,
) -> Canvas {
    let window_size = window::get_size(ctx);
    let window_min_size = i32::min(window_size.0, window_size.1);
    let zoom = (window_min_size as f32 - 10.0) / size as f32;
    let pixel_size = window_min_size - 10;
    let canvas = Canvas::new(ctx, pixel_size, pixel_size).unwrap();
    graphics::set_canvas(ctx, &canvas);
    let mesh = Mesh::rounded_rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(0.0, 0.0, pixel_size as f32, pixel_size as f32),
        BorderRadii::new(5.0),
    )
    .unwrap();
    mesh.draw(ctx, DrawParams::new().color(Colors::SPACE_VIOLET));

    // TODO: use some textures for beauty
    let mut data = Vec::with_capacity(size * size * 4);
    let max = 262_144.0f32;
    for y in 0..size {
        for x in 0..size {
            let i = x * size + y;
            let d = quadrants[i] as f32 / max;
            data.push(255);
            data.push(255);
            data.push(224);
            data.push((255.0 * d).round() as u8);
        }
    }
    let texture = Texture::from_data(
        ctx,
        size as i32,
        size as i32,
        TextureFormat::Rgba8,
        data.as_slice(),
    )
    .unwrap();
    texture.draw(ctx, DrawParams::new().scale(Vec2::new(zoom, zoom)));
    let mesh = Mesh::rounded_rectangle(
        ctx,
        ShapeStyle::Stroke(2.0),
        Rectangle::new(0.0, 0.0, pixel_size as f32, pixel_size as f32),
        BorderRadii::new(5.0),
    )
    .unwrap();
    mesh.draw(ctx, DrawParams::new().color(Colors::ORANGE));

    let mut text = Text::new(format!("{} galaxy", name), font_title.font);
    let bounds = text.get_bounds(ctx).unwrap();
    text.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(
                (pixel_size as f32) / 2.0 - bounds.width / 2.0,
                10.0,
            ))
            .color(Colors::ORANGE),
    );
    let sum: u64 = quadrants.iter().copied().map(u64::from).sum();
    let mut text = Text::new(
        format!("{} stars in {}x{} quadrants", sum, size, size),
        font_bottom.font,
    );
    let bounds = text.get_bounds(ctx).unwrap();
    text.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(
                (pixel_size as f32) / 2.0 - bounds.width / 2.0,
                pixel_size as f32 - bounds.height - 10.0,
            ))
            .color(Colors::LIGHT_GRAY),
    );

    graphics::reset_canvas(ctx);
    canvas
}

pub struct Galaxy {
    canvas: Option<Canvas>,
    font_title: PreparedFont,
    font_bottom: PreparedFont,
    size: usize,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Galaxy {
    pub fn new(
        size: usize,
        font_title: PreparedFont,
        font_bottom: PreparedFont,
        position: Position,
    ) -> Self {
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

    pub fn redraw(&mut self, ctx: &mut Context, size: usize, quadrants: Vec<u32>, name: &str) {
        self.size = size;
        self.canvas = Some(draw_galaxy(
            ctx,
            size,
            &quadrants,
            name,
            self.font_title.clone(),
            self.font_bottom.clone(),
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

impl Focus for Galaxy {
    fn focused(&self) -> bool {
        self.visible()
    }

    fn set_focused(&mut self, focused: bool) {
        self.set_visible(focused)
    }
}

impl UiSprite for Galaxy {}
