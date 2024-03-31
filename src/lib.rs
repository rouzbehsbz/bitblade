mod engine;
mod formatter;
mod painter;
mod space;
mod terminal;

#[cfg(test)]
mod tests {
    use crate::{
        engine::Engine, formatter::Color, painter::Painter, space::Vec2, terminal::Window,
    };

    #[test]
    fn it_works() {
        let mut app = Engine::default();

        app.run(|window: &mut Window| {
            Painter::new(&mut window.canvas)
                .set_foreground(Color::Red)
                .set_background(Color::Cyan)
                .move_origin(Vec2(2, 5))
                .draw_text("salam")
                .reset_origin()
                .draw_text("Hiiii");
        })
        .unwrap();
    }
}
