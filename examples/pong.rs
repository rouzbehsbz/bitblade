use bitblade::{
    engine::{Engine, State},
    formatter::{Color, Style},
    interface::{KeyCode, Keyboard},
    painter::Painter,
    space::Vec2,
    terminal::Window,
};

use rand::{self, Rng};

fn main() {
    let mut engine = Engine::default();

    let mut game = Game::new(Vec2(3, 4), Vec2(56, 4), Vec2(30, 5));

    engine
        .run(|state: &State, window: &mut Window, keyboard: &Keyboard| {
            let borders_dimension = window.canvas.dimension;

            if state.get_tick() % 10 == 0 {
                game.update(borders_dimension, keyboard);
            }

            let scores_text = format!("{} - {}", game.player1.score, game.player2.score);

            Painter::new(&mut window.canvas)
                // Draw Ball
                .set_foreground(Color::Red)
                .set_style(Style::Bold)
                .draw_char_at_pos(game.ball.pos, '*')
                // Draw Players
                .set_background(Color::Green)
                .set_style(Style::Normal)
                .set_origin(game.player1.pos)
                .draw_vertical_line(game.player1.length, ' ')
                .set_origin(game.player2.pos)
                .draw_vertical_line(game.player2.length, ' ')
                // Draw Scores
                .set_background(Color::Black)
                .set_foreground(Color::Cyan)
                .set_origin(Vec2(
                    (borders_dimension.0 - (scores_text.len() / 2) as i32) / 2,
                    1,
                ))
                .draw_text(&scores_text)
                // Draw Borders
                .reset_origin()
                .set_foreground(Color::Yellow)
                .draw_rectangle(
                    Vec2(borders_dimension.0 - 1, borders_dimension.1 - 1),
                    '-',
                    '|',
                );
        })
        .unwrap();
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn get_random_horizontal() -> Self {
        match rand::thread_rng().gen_range(0..=1) {
            0 => Self::Right,
            1 => Self::Left,
            _ => Self::Right,
        }
    }

    fn get_random_vertical() -> Self {
        match rand::thread_rng().gen_range(0..=1) {
            0 => Self::Up,
            1 => Self::Down,
            _ => Self::Up,
        }
    }
}

pub struct Ball {
    pub pos: Vec2,
    pub initial_pos: Vec2,
    pub h_direction: Direction,
    pub v_direction: Direction,
}

impl Ball {
    pub fn new(initial_pos: Vec2) -> Self {
        Self {
            pos: initial_pos,
            initial_pos,
            h_direction: Direction::get_random_horizontal(),
            v_direction: Direction::get_random_vertical(),
        }
    }

    pub fn update_pos(&mut self, borders_dimension: Vec2, player1: &Player, player2: &Player) {
        if self.pos.0 <= player1.pos.0
            && self.pos.1 >= player1.pos.1
            && self.pos.1 <= player1.pos.1 + player1.length
        {
            self.h_direction = Direction::Right;
        }
        if self.pos.0 >= player2.pos.0
            && self.pos.1 >= player2.pos.1
            && self.pos.1 <= player2.pos.1 + player2.length
        {
            self.h_direction = Direction::Left;
        }
        if self.pos.1 <= 1 {
            self.v_direction = Direction::Down;
        }
        if self.pos.1 >= borders_dimension.1 - 2 {
            self.v_direction = Direction::Up;
        }

        match self.h_direction {
            Direction::Right => self.pos.0 += 1,
            Direction::Left => self.pos.0 -= 1,
            _ => {}
        }

        match self.v_direction {
            Direction::Up => self.pos.1 -= 1,
            Direction::Down => self.pos.1 += 1,
            _ => {}
        }
    }
}

pub struct Player {
    pub is_first_player: bool,
    pub pos: Vec2,
    pub length: i32,
    pub score: u32,
}

impl Player {
    pub fn new(is_first_player: bool, initial_pos: Vec2, length: i32) -> Self {
        Self {
            is_first_player,
            pos: initial_pos,
            length,
            score: 0,
        }
    }

    pub fn update_pos(&mut self, borders_dimension: Vec2, keyboard: &Keyboard) {
        if self.is_first_player {
            if keyboard.is_key_pressed(&KeyCode::W) {
                self.pos.1 -= 1;
            } else if keyboard.is_key_pressed(&KeyCode::S) {
                self.pos.1 += 1;
            }
        } else {
            if keyboard.is_key_pressed(&KeyCode::Up) {
                self.pos.1 -= 1;
            } else if keyboard.is_key_pressed(&KeyCode::Down) {
                self.pos.1 += 1;
            }
        }

        if self.pos.1 <= 1 {
            self.pos.1 = 1;
        } else if self.pos.1 + self.length >= borders_dimension.1 - 1 {
            self.pos.1 = borders_dimension.1 - 1 - self.length;
        }
    }
}

pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub ball: Ball,
}

impl Game {
    pub fn new(
        player1_initial_pos: Vec2,
        player2_initial_pos: Vec2,
        ball_initial_pos: Vec2,
    ) -> Self {
        Self {
            player1: Player::new(true, player1_initial_pos, 2),
            player2: Player::new(false, player2_initial_pos, 2),
            ball: Ball::new(ball_initial_pos),
        }
    }

    pub fn detect_goal(&mut self, borders_dimension: Vec2) {
        if self.ball.pos.0 <= 0 {
            self.player2.score += 1;

            self.ball = Ball::new(self.ball.initial_pos)
        } else if self.ball.pos.0 >= borders_dimension.0 {
            self.player2.score += 1;

            self.ball = Ball::new(self.ball.initial_pos)
        }
    }

    pub fn update(&mut self, borders_dimension: Vec2, keyboard: &Keyboard) {
        self.player1.update_pos(borders_dimension, keyboard);
        self.player2.update_pos(borders_dimension, keyboard);
        self.ball
            .update_pos(borders_dimension, &self.player1, &self.player2);

        self.detect_goal(borders_dimension);
    }
}
