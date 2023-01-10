use crate::assets::{Assets, PreparedFont, TileSet};
use crate::colors::Colors;
use crate::input;
use crate::scenes::Transition;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Disable, Draw, Hover, Positionate, Press, Sprite, Update};
use geometry::{Rect, Vec2};
use std::rc::Rc;
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::Context;

enum ButtonContent {
    Text(Text, f32),
    Empty(Vec2),
    AsciiIcon(char, Rc<Assets>, f32),
}

impl ButtonContent {
    pub const fn offset_x(&self) -> f32 {
        match self {
            ButtonContent::Text(..) | ButtonContent::AsciiIcon(..) => 20.0,
            ButtonContent::Empty(..) => 0.0,
        }
    }
}

enum ButtonState {
    Default,
    Pressed,
    Hovered,
    Disabled,
}

impl ButtonState {
    fn fg_color(&self) -> Color {
        match self {
            ButtonState::Default => Colors::ORANGE,
            ButtonState::Pressed => Colors::SPACE_VIOLET,
            ButtonState::Hovered => Colors::ORANGE,
            ButtonState::Disabled => Colors::DARK_GRAY,
        }
    }

    fn bg_color(&self) -> Color {
        match self {
            ButtonState::Default => Colors::SPACE_VIOLET,
            ButtonState::Pressed => Colors::ORANGE,
            ButtonState::Hovered => Colors::DARK_ORANGE_RED,
            ButtonState::Disabled => Colors::GRAY,
        }
        .with_alpha(0.8)
    }

    fn border_color(&self) -> Color {
        match self {
            ButtonState::Default => Colors::ORANGE,
            ButtonState::Pressed => Colors::DARK_ORANGE_RED,
            ButtonState::Hovered => Colors::ORANGE,
            ButtonState::Disabled => Colors::DARK_GRAY,
        }
    }
}

pub struct Button {
    keys: Vec<(Key, Option<KeyModifier>)>,
    content: ButtonContent,
    on_click: Transition,
    position: Position,
    border: Option<Mesh>,
    bg: Option<Mesh>,
    rect: Option<Rect>,
    is_pressed: bool,
    is_disabled: bool,
    is_hovered: bool,
    fixable: bool,
    visible: bool,
}

impl Button {
    fn new(
        keys: Vec<(Key, Option<KeyModifier>)>,
        content: ButtonContent,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self {
            keys,
            content,
            on_click,
            position,
            border: None,
            bg: None,
            rect: None,
            is_pressed: false,
            is_hovered: false,
            is_disabled: false,
            fixable: false,
            visible: true,
        }
    }

    pub fn text(
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        font: PreparedFont,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(
            keys,
            ButtonContent::Text(Text::new(text, font.font), font.line_height),
            position,
            on_click,
        )
    }

    pub fn empty(
        keys: Vec<(Key, Option<KeyModifier>)>,
        size: Vec2,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(keys, ButtonContent::Empty(size), position, on_click)
    }

    pub fn icon(
        keys: Vec<(Key, Option<KeyModifier>)>,
        ch: char,
        assets: Rc<Assets>,
        zoom: f32,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(
            keys,
            ButtonContent::AsciiIcon(ch, assets, zoom),
            position,
            on_click,
        )
    }

    pub fn fixed(
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        font: PreparedFont,
        state: bool,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self {
            fixable: true,
            is_pressed: state,
            ..Self::text(keys, text, font, position, on_click)
        }
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.is_disabled = val;
        self
    }

    fn content_size(&mut self, ctx: &mut Context) -> Vec2 {
        match &mut self.content {
            ButtonContent::Text(text, height) => text
                .get_bounds(ctx)
                .map(|b| Vec2::new(b.width, *height))
                .unwrap(),
            ButtonContent::Empty(size) => *size,
            ButtonContent::AsciiIcon(.., zoom) => {
                Vec2::new(TileSet::TILE_SIZE.0 as f32, TileSet::TILE_SIZE.1 as f32) * (*zoom)
            }
        }
    }

    fn state(&self) -> ButtonState {
        if self.is_disabled {
            ButtonState::Disabled
        } else if self.is_pressed {
            ButtonState::Pressed
        } else if self.is_hovered {
            ButtonState::Hovered
        } else {
            ButtonState::Default
        }
    }
}

impl Draw for Button {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut vec = Vec2::new(rect.x, rect.y);
        let content_size = self.content_size(ctx);
        let bg_color = self.state().bg_color();
        self.bg
            .as_mut()
            .unwrap()
            .draw(ctx, DrawParams::new().position(vec).color(bg_color));
        let border_color = self.state().border_color();
        self.border
            .as_mut()
            .unwrap()
            .draw(ctx, DrawParams::new().position(vec).color(border_color));
        vec += Vec2::new(rect.w, rect.h) / 2.0 - content_size / 2.0;
        // get a copy because mutable borrow below
        let color = self.state().fg_color();
        if let ButtonContent::Text(text, _) = &mut self.content {
            // hack for "[key] Name" buttons
            if text.content().starts_with('[') {
                vec.x -= 2.0;
            }
            text.draw(ctx, DrawParams::new().position(vec).color(color));
        } else if let ButtonContent::AsciiIcon(ch, assets, zoom) = &self.content {
            assets.tileset.draw(
                ctx,
                *ch,
                DrawParams::new()
                    .position(vec)
                    .color(color)
                    .scale(Vec2::new(*zoom, *zoom)),
            );
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Button {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let content_size = self.content_size(ctx);
        let offset_x = self.content.offset_x();
        self.border = Some(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Stroke(2.0),
                Rectangle::new(0.0, 0.0, content_size.x + offset_x, 42.0),
                BorderRadii::new(5.0),
            )
            .unwrap(),
        );
        self.bg = Some(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, content_size.x + offset_x, 42.0),
                BorderRadii::new(5.0),
            )
            .unwrap(),
        );
        Vec2::new(content_size.x + offset_x, 42.0)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Button {
    fn update(&mut self, ctx: &mut Context, focused: bool, blocked: &[Rect]) -> Option<Transition> {
        if self.is_disabled {
            return None;
        }
        if !self.keys.is_empty() && !focused {
            let mut on_pressed = false;
            let mut off_pressed = false;
            for (key, key_mod) in self.keys.iter().copied() {
                if input::is_key_with_mod_pressed(ctx, (key, key_mod)) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx, key) && self.is_pressed {
                    off_pressed = true
                }
            }
            if on_pressed {
                self.on_pressed();
            } else if off_pressed {
                self.off_pressed();
                return Some(self.on_click.clone());
            }
        }
        let mouse = input::get_mouse_position(ctx);
        let rect = self.rect.unwrap();
        let collides = rect.contains_point(mouse);
        if collides && blocked.iter().any(|r| r.contains_point(mouse)) {
            return None;
        }
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        if collides && !self.is_pressed && input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            self.on_pressed();
        } else if self.is_pressed && input::is_mouse_button_released(ctx, MouseButton::Left) {
            self.off_pressed();
            if collides {
                return Some(self.on_click.clone());
            }
        }
        None
    }
}

impl Disable for Button {
    fn disabled(&self) -> bool {
        self.is_disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        if disabled != self.is_disabled {
            self.is_disabled = disabled;
        }
    }
}

impl Hover for Button {
    fn on_hovered(&mut self) {
        self.is_hovered = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
    }
}

impl Press for Button {
    fn on_pressed(&mut self) {
        self.is_pressed = true;
    }

    fn off_pressed(&mut self) {
        if !self.fixable {
            self.unpress();
        }
    }

    fn unpress(&mut self) {
        self.is_pressed = false;
    }
}

impl Sprite for Button {}
