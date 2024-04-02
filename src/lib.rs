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

        struct Ball(i32, i32);

        let mut ball = Ball(8, 3);
        let mut is_direction_up = true;
        let mut counter = 0;

        app.run(|window: &mut Window| {
            if counter % 1 == 0 {
                if is_direction_up {
                    ball.1 -= 1;
                } else {
                    ball.1 += 1;
                }
            }

            if ball.1 <= 0 {
                is_direction_up = false;
            }
            if ball.1 >= window.canvas.dimension.1 - 1 {
                is_direction_up = true;
            }

            counter += 1;

            Painter::new(&mut window.canvas)
                .set_foreground(Color::Yellow)
                .draw_text("FPS: 60")
                .set_foreground(Color::Red)
                .draw_char_at_pos(Vec2(ball.0, ball.1), 'o');
        })
        .unwrap();
    }
}
