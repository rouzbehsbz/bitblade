use crate::{
    formatter::{Color, Style},
    space::Vec2,
    terminal::Canvas,
};

pub struct Painter<'a> {
    canvas: &'a mut Canvas,
    origin: Vec2,
    style: Style,
    foreground: Color,
    background: Color,
}

impl<'a> Painter<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        Self {
            origin: Vec2::zero(),
            foreground: canvas.get_default_element().foreground,
            background: canvas.get_default_element().background,
            style: canvas.get_default_element().style,
            canvas,
        }
    }

    pub fn set_origin(&mut self, pos: Vec2) -> &mut Self {
        self.origin = pos;

        self
    }

    pub fn reset_origin(&mut self) -> &mut Self {
        self.origin = Vec2::zero();

        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;

        self
    }

    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.background = color;

        self
    }

    pub fn set_foreground(&mut self, color: Color) -> &mut Self {
        self.foreground = color;

        self
    }

    pub fn move_origin(&mut self, vec: Vec2) -> &mut Self {
        self.origin += vec;

        self
    }

    pub fn draw_char_at_pos(&mut self, pos: Vec2, value: char) -> &mut Self {
        if let Some(element) = self.canvas.get_cell_mut(pos) {
            element.value = value;
            element.style = self.style;
            element.background = self.background;
            element.foreground = self.foreground;
        }

        self
    }

    pub fn draw(&mut self, value: char) -> &mut Self {
        self.draw_char_at_pos(self.origin, value);

        self
    }

    pub fn draw_text(&mut self, text: &str) -> &mut Self {
        for (index, value) in text.chars().enumerate() {
            let pos = Vec2(self.origin.0 + index as i32, self.origin.1);

            self.draw_char_at_pos(pos, value);
        }

        self
    }

    pub fn draw_horizontal_line(&mut self, count: i32, value: char) -> &mut Self {
        for index in 0..count.abs() {
            if count > 0 {
                let pos = Vec2(self.origin.0 + index, self.origin.1);

                self.draw_char_at_pos(pos, value);
            } else {
                let pos = Vec2(self.origin.0 - index, self.origin.1);

                self.draw_char_at_pos(pos, value);
            }
        }

        self
    }

    pub fn draw_vertical_line(&mut self, count: i32, value: char) -> &mut Self {
        for index in 0..count.abs() {
            if count > 0 {
                let pos = Vec2(self.origin.0, self.origin.1 + index);

                self.draw_char_at_pos(pos, value);
            } else {
                let pos = Vec2(self.origin.0, self.origin.1 - index);

                self.draw_char_at_pos(pos, value);
            }
        }

        self
    }
}
