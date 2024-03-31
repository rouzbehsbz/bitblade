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

    pub fn set_origin(&'a mut self, pos: Vec2) -> &'a mut Self {
        self.origin = pos;

        self
    }

    pub fn set_style(&'a mut self, style: Style) -> &'a mut Self {
        self.style = style;

        self
    }

    pub fn set_background(&'a mut self, color: Color) -> &'a mut Self {
        self.background = color;

        self
    }

    pub fn set_foreground(&'a mut self, color: Color) -> &'a mut Self {
        self.foreground = color;

        self
    }

    pub fn move_origin(&'a mut self, vec: Vec2) -> &'a mut Self {
        self.origin += vec;

        self
    }

    pub fn draw_element(&'a mut self, pos: Vec2, value: char) {
        if let Some(element) = self.canvas.get_cell_mut(pos) {
            element.value = value;
            element.foreground = self.foreground;
            element.background = self.background;
            element.style = self.style;
        }
    }
}
