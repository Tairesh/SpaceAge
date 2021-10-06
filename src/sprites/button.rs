use crate::colors::Colors;
use crate::scenes::Transition;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Disable, Draw, Hover, Positionate, Press, Sprite, Update};
use crate::{Rect, Vec2};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

enum ButtonContent {
    Text(Text),
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
    id: String,
    keys: Vec<(Key, Option<KeyModifier>)>,
    content: ButtonContent,
    on_click: Transition,
    content_height: f32,
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
    pub fn new(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        position: Position,
        font: Font,
        on_click: Transition,
    ) -> Self {
        Self {
            id: id.to_string(),
            keys,
            content: ButtonContent::Text(Text::new(text, font)),
            on_click,
            content_height: 20.0,
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

    pub fn fixed(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        state: bool,
        position: Position,
        font: Font,
        on_click: Transition,
    ) -> Self {
        let mut s = Self::new(id, keys, text, position, font, on_click);
        s.fixable = true;
        s.is_pressed = state;
        s
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.is_disabled = val;
        self
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    fn content_size(&mut self, ctx: &mut Context) -> (Vec2, f32) {
        match &mut self.content {
            ButtonContent::Text(text) => (
                text.get_bounds(ctx)
                    .map(|b| Vec2::new(b.width, self.content_height))
                    .unwrap(),
                30.0f32,
            ),
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
        let (content_size, _) = self.content_size(ctx);
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
        vec.x += rect.w / 2.0 - content_size.x / 2.0;
        vec.y += rect.h / 2.0 - content_size.y / 2.0 - 2.0;
        // if !self.is_pressed {
        //     vec.y -= 2.0;
        // }
        let text_color = self.state().fg_color();
        match &mut self.content {
            ButtonContent::Text(text) => {
                if !self.keys.is_empty() {
                    vec.x -= 3.0;
                }
                text.draw(ctx, DrawParams::new().position(vec).color(text_color));
            }
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
        let (content_size, offset_x) = self.content_size(ctx);
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

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

fn is_pressed_key_with_mod(ctx: &mut Context, key: Key, key_mod: Option<KeyModifier>) -> bool {
    if !input::is_key_pressed(ctx, key) {
        return false;
    }
    if let Some(key_mod) = key_mod {
        input::is_key_modifier_down(ctx, key_mod)
    } else {
        !input::is_key_modifier_down(ctx, KeyModifier::Alt)
            && !input::is_key_modifier_down(ctx, KeyModifier::Ctrl)
            && !input::is_key_modifier_down(ctx, KeyModifier::Shift)
    }
}

impl Update for Button {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if self.is_disabled {
            return None;
        }
        if !self.keys.is_empty() {
            let mut on_pressed = false;
            let mut off_pressed = false;
            for (key, key_mod) in self.keys.iter() {
                if is_pressed_key_with_mod(ctx, *key, *key_mod) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx, *key) && self.is_pressed {
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
