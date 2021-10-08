#![allow(dead_code)]
use crate::colors::Colors;
use crate::scenes::Transition;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Disable, Draw, Hover, Positionate, Press, Sprite, Stringify, Update};
use crate::{Rect, Vec2};
use std::time::{Duration, Instant};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

enum ValueType {
    String { max_length: u32 },
    Unsigned { min: u32, max: u32 },
}

pub struct TextInput {
    text: Text,
    text_with_spaces: Text,
    position: Position,
    width: f32,
    value_type: ValueType,
    rect: Option<Rect>,
    is_focused: bool,
    is_disabled: bool,
    is_hovered: bool,
    is_danger: bool,
    blink: bool,
    last_blinked: Instant,
    visible: bool,
    bg: Option<Mesh>,
    border: Option<Mesh>,
    cursor: Option<Mesh>,
}

impl TextInput {
    pub fn new<C: Into<String>>(value: C, width: f32, font: Font, position: Position) -> Self {
        let value = value.into();
        Self {
            value_type: ValueType::String { max_length: 16 },
            text: Text::new(value.clone(), font.clone()),
            text_with_spaces: Text::new(value.replace(" ", "_"), font),
            position,
            width,
            rect: None,
            is_focused: false,
            is_disabled: false,
            is_hovered: false,
            is_danger: false,
            blink: false,
            last_blinked: Instant::now(),
            visible: true,
            bg: None,
            border: None,
            cursor: None,
        }
    }

    pub fn int(value: u32, clamps: (u32, u32), width: f32, font: Font, position: Position) -> Self {
        let mut s = Self::new(format!("{}", value).as_str(), width, font, position);
        s.value_type = ValueType::Unsigned {
            min: clamps.0,
            max: clamps.1,
        };
        s
    }

    fn border_color(&self) -> Color {
        if self.is_danger {
            Colors::RED
        } else if self.is_disabled {
            Colors::GRAY
        } else if self.is_focused {
            Colors::ORANGE_RED
        } else {
            Colors::DARK_ORANGE_RED
        }
    }

    fn bg_color(&self) -> Option<Color> {
        if self.is_danger && self.is_focused {
            Some(Colors::DARK_RED.with_alpha(0.8))
        } else if self.is_disabled {
            Some(Colors::DARK_GRAY.with_alpha(0.8))
        } else if self.is_focused || self.is_hovered {
            Some(Colors::SPACE_VIOLET.with_alpha(0.8))
        } else {
            None
        }
    }

    fn text_color(&self) -> Color {
        if self.is_disabled {
            Colors::WHITE
        } else if self.is_focused {
            Colors::LIGHT_YELLOW
        } else {
            Colors::ORANGE_RED
        }
    }

    fn validate_value(&mut self) {
        if let ValueType::Unsigned { min, max } = self.value_type {
            let mut val = self.text.content().parse::<u32>().unwrap_or(min);
            if val < min {
                val = min;
            } else if val > max {
                val = max;
            }
            self.text.set_content(format!("{}", val));
        }
    }

    pub fn set_danger(&mut self, danger: bool) {
        if danger != self.is_danger {
            self.is_danger = danger;
        }
    }

    pub fn danger(&self) -> bool {
        self.is_danger
    }
}

impl Draw for TextInput {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        if let Some(bg_color) = self.bg_color() {
            self.bg.as_ref().unwrap().draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(rect.x, rect.y))
                    .color(bg_color),
            );
        }
        self.border.as_ref().unwrap().draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .color(self.border_color()),
        );
        let text_width = self
            .text_with_spaces
            .get_bounds(ctx)
            .map(|r| r.width + 3.0)
            .unwrap_or(-1.0f32);
        let y = (rect.y + rect.h / 2.0 - 15.0).round();
        let text_pos = if !self.is_focused || self.is_disabled {
            Vec2::new(rect.x + rect.w / 2.0 - text_width / 2.0, y)
        } else {
            Vec2::new(rect.x + 7.0, y)
        };
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(text_pos)
                .color(self.text_color()),
        );
        if self.blink && self.is_focused {
            self.cursor.as_ref().unwrap().draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        rect.x + text_width + 10.0,
                        rect.y + rect.h / 2.0 - 15.0,
                    ))
                    .color(self.text_color()),
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

impl Positionate for TextInput {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let (w, h) = (self.width, 42.0);
        self.bg = Some(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, w, h),
                BorderRadii::new(5.0),
            )
            .unwrap(),
        );
        self.border = Some(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Stroke(2.0),
                Rectangle::new(0.0, 0.0, w, h),
                BorderRadii::new(5.0),
            )
            .unwrap(),
        );
        self.cursor = Some(
            Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 10.0, 30.0)).unwrap(),
        );
        Vec2::new(w, h)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for TextInput {
    fn update(
        &mut self,
        ctx: &mut Context,
        _focused: bool,
        blocked: &[Rect],
    ) -> Option<Transition> {
        let mouse = input::get_mouse_position(ctx);
        let collides = self.rect.unwrap().contains_point(mouse);
        if collides && blocked.iter().any(|r| r.contains_point(mouse)) {
            return None;
        }
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        if self.is_focused {
            if input::is_mouse_button_pressed(ctx, MouseButton::Left) && !collides {
                self.off_pressed();
            }
            if Instant::now() - self.last_blinked > Duration::new(0, 500_000_000) {
                self.blink = !self.blink;
                self.last_blinked = Instant::now();
            }
            if input::is_key_pressed(ctx, Key::Backspace) && !self.text.content().is_empty() {
                self.text.pop();
                self.text_with_spaces.pop();
                self.is_danger = false;
            }
            if let Some(text_input) = input::get_text_input(ctx) {
                let allow = match self.value_type {
                    ValueType::String { max_length } => {
                        (self.text.content().len() + text_input.len()) as u32 <= max_length
                    }
                    ValueType::Unsigned { .. } => matches!(text_input.parse::<u32>(), Ok(_)),
                };
                if allow {
                    self.text.push_str(text_input);
                    self.text_with_spaces
                        .push_str(text_input.to_string().replace(" ", "_").as_str());
                    self.is_danger = false;
                }
            }
            if let ValueType::String { max_length } = self.value_type {
                if (input::is_key_pressed(ctx, Key::V)
                    && input::is_key_modifier_down(ctx, KeyModifier::Ctrl))
                    || (input::is_key_pressed(ctx, Key::Insert)
                        && input::is_key_modifier_down(ctx, KeyModifier::Shift))
                {
                    let clipboard: String = input::get_clipboard_text(ctx)
                        .unwrap()
                        .chars()
                        .map(|c| if c == '\n' { ' ' } else { c })
                        .collect();
                    self.text.push_str(clipboard.as_str());
                    while self.text.content().len() as u32 > max_length {
                        self.text.pop();
                        self.text_with_spaces.pop();
                    }
                    self.is_danger = false;
                }
            }
        } else if input::is_mouse_button_pressed(ctx, MouseButton::Left)
            && collides
            && !self.is_disabled
        {
            self.on_pressed();
        }
        None
    }
}

impl Disable for TextInput {
    fn disabled(&self) -> bool {
        self.is_disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        if disabled != self.is_disabled {
            self.is_disabled = disabled;
        }
    }
}

impl Stringify for TextInput {
    fn value(&self) -> String {
        self.text.content().to_string()
    }

    fn set_value<C: Into<String>>(&mut self, value: C) {
        self.text.set_content(value);
        self.text_with_spaces
            .set_content(self.text.content().replace(" ", "_"));
        self.is_danger = false;
        self.validate_value();
    }
}

impl Hover for TextInput {
    fn on_hovered(&mut self) {
        self.is_hovered = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
    }
}

impl Press for TextInput {
    fn on_pressed(&mut self) {
        self.is_focused = true;
        self.blink = true;
        self.last_blinked = Instant::now();
    }

    fn off_pressed(&mut self) {
        self.unpress();
    }

    fn unpress(&mut self) {
        self.is_focused = false;
        self.blink = false;
        self.validate_value();
    }
}

impl Sprite for TextInput {
    fn focused(&self) -> bool {
        self.is_focused
    }

    fn set_focused(&mut self, focused: bool) {
        if focused {
            self.on_pressed();
        } else {
            self.off_pressed();
        }
    }
}
