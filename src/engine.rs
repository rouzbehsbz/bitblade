use std::{
    io::Result,
    ops::Sub,
    thread,
    time::{Duration, Instant},
};

use crate::{interface::Keyboard, terminal::Window};

pub struct Config {
    tick_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::new(16)
    }
}

impl Config {
    pub fn new(tick_rate: u64) -> Self {
        Self { tick_rate }
    }
}

pub struct Engine {
    window: Window,
    keyboard: Keyboard,
    config: Config,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Config::default(), Window::default(), Keyboard::new())
    }
}

impl Engine {
    pub fn new(config: Config, window: Window, keyboard: Keyboard) -> Self {
        Self {
            config,
            window,
            keyboard,
        }
    }

    pub fn run<F>(&mut self, mut tick_update: F) -> Result<()>
    where
        F: FnMut(&mut Window, &Keyboard),
    {
        let tick_rate_duration = Duration::from_millis(self.config.tick_rate);
        let mut last_time = Instant::now();

        loop {
            let elapsed_time = last_time.elapsed();

            last_time = Instant::now();

            self.window.clear()?;
            self.keyboard.poll_keys();

            tick_update(&mut self.window, &self.keyboard);

            self.window.draw()?;

            let sleep_time = tick_rate_duration
                .checked_sub(elapsed_time)
                .unwrap_or_else(|| elapsed_time.sub(tick_rate_duration));

            thread::sleep(sleep_time)
        }
    }
}
