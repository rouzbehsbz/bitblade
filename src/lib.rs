mod engine;
mod formatter;
mod painter;
mod space;
mod terminal;

#[cfg(test)]
mod tests {
    use crate::{engine::Engine, painter::Painter, space::Vec2, terminal::Window};

    #[test]
    fn it_works() {
        let mut app = Engine::default();

        app.run(|window: &mut Window| {
            Painter::new(&mut window.canvas)
                .set_foreground(crate::formatter::Color::Red)
                .set_background(crate::formatter::Color::Cyan)
                .draw_element(Vec2(2, 2), '3')
        })
        .unwrap();
    }
}
