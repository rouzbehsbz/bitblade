use std::io::{BufWriter, Write};

use crate::space::Vec2;

#[derive(Clone, Copy)]
pub struct Element {
    value: char,
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
    target: BufWriter<W>,
}

impl<W> View<W>
where
    W: Write,
{
    fn new(target: W) -> Self {
        Self {
            target: BufWriter::new(target),
        }
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

    pub fn draw(&self) {
        let columns = self.canvas.dimension.0;
        let rows = self.canvas.dimension.1;

        for row in 0..rows {
            for column in 0..columns {
                let cell = self.canvas.get_cell(Vec2(column, row));

                if let Some(element) = cell {}
            }
        }
    }
}
