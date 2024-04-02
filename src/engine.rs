use std::{
    io::Result,
    num::Wrapping,
    ops::{Add, Sub},
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

pub struct State {
    tick: Wrapping<u32>,
}

impl State {
    pub fn new() -> Self {
        Self { tick: Wrapping(0) }
    }

    pub fn increase_tick(&mut self) {
        self.tick += 1;
    }

    pub fn get_tick(&self) -> u32 {
        self.tick.0
    }
}

pub struct Engine {
    state: State,
    window: Window,
    keyboard: Keyboard,
    config: Config,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Config::default(), Window::default())
    }
}

impl Engine {
    pub fn new(config: Config, window: Window) -> Self {
        Self {
            state: State::new(),
            config,
            window,
            keyboard: Keyboard::new(),
        }
    }

    pub fn run<F>(&mut self, mut tick_update: F) -> Result<()>
    where
        F: FnMut(&State, &mut Window, &Keyboard),
    {
        let tick_rate_duration = Duration::from_millis(self.config.tick_rate);
        let mut last_time = Instant::now();

        loop {
            let elapsed_time = last_time.elapsed();

            last_time = Instant::now();

            self.window.clear()?;
            self.keyboard.poll_keys();

            tick_update(&mut self.state, &mut self.window, &self.keyboard);

            self.window.draw()?;
            self.state.increase_tick();

            let sleep_time = tick_rate_duration
                .checked_sub(elapsed_time)
                .unwrap_or_else(|| elapsed_time.sub(tick_rate_duration));

            thread::sleep(sleep_time)
        }
    }
}
