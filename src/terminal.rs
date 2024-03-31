use std::io::{BufWriter, Result, Write};

use crate::{
    formatter::{Color, Style},
    space::Vec2,
};

const NEW_LINE_CHARACTER: char = '\n';
const CLEAR_SCREEN_ANSI_ESCAPE_STR: &str = "\x1B[2J";
const MOVE_CURSOR_TO_ORIGIN_ANSI_ESCAPE_STR: &str = "\x1B[1;1H";

#[derive(Clone, Copy)]
pub struct Element {
    value: char,
    style: Style,
    background: Color,
    foreground: Color,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            value: ' ',
            style: Style::Normal,
            background: Color::Black,
            foreground: Color::White,
        }
    }
}

pub struct Canvas {
    cells: Vec<Element>,
    dimension: Vec2,
}

impl Canvas {
    fn new(dimension: Vec2, default_element: Element) -> Self {
        let mut cells = Vec::new();

        cells.resize((dimension.0 * dimension.1) as usize, default_element);

        Self { cells, dimension }
    }

    fn is_cell_exists(&self, pos: Vec2) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.dimension.0 && pos.1 < self.dimension.1
    }

    fn get_cell(&self, pos: Vec2) -> Option<&Element> {
        if self.is_cell_exists(pos) {
            Some(&self.cells[(pos.1 * self.dimension.0 + pos.0) as usize])
        } else {
            None
        }
    }
}

struct View<W>
where
    W: Write,
{
    last_element: Element,
    target: BufWriter<W>,
}

impl<W> View<W>
where
    W: Write,
{
    fn new(target: W) -> Self {
        Self {
            target: BufWriter::new(target),
            last_element: Element::default(),
        }
    }

    fn write_element(&mut self, element: &Element) -> Result<usize> {
        if self.last_element.background != element.background {
            self.write_str(element.background.to_background_ansi_escape_code())?;
        }
        if self.last_element.foreground != element.foreground {
            self.write_str(element.background.to_foreground_ansi_escape_code())?;
        }
        if self.last_element.style != element.style {
            self.write_str(element.style.to_ansi_escape_code())?;
        }

        self.write_char(element.value)
    }

    fn write_char(&mut self, character: char) -> Result<usize> {
        let byte_format: u8 = character as u8;

        self.target.write(&[byte_format])
    }

    fn write_str(&mut self, string: &str) -> Result<usize> {
        let byte_format = string.as_bytes();

        self.target.write(byte_format)
    }

    fn flush(&mut self) -> Result<()> {
        self.target.flush()
    }
}

pub struct Window<W>
where
    W: Write,
{
    canvas: Canvas,
    view: View<W>,
}

impl<W> Window<W>
where
    W: Write,
{
    pub fn new(canvas: Canvas, view: View<W>) -> Self {
        Self { canvas, view }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.view.write_str(CLEAR_SCREEN_ANSI_ESCAPE_STR)?;
        self.view.write_str(MOVE_CURSOR_TO_ORIGIN_ANSI_ESCAPE_STR)?;

        self.view.flush()
    }

    pub fn draw(&mut self) -> Result<()> {
        let columns = self.canvas.dimension.0;
        let rows = self.canvas.dimension.1;

        for row in 0..rows {
            for column in 0..columns {
                let cell = self.canvas.get_cell(Vec2(column, row));

                if let Some(element) = cell {
                    self.view.write_element(element)?;
                }
            }

            self.view.write_char(NEW_LINE_CHARACTER)?;
        }

        self.view.flush()
    }
}
