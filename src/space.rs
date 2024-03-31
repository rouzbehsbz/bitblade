#[derive(Clone, Copy)]
pub struct Vec2(pub i32, pub i32);

impl Vec2 {
    pub fn zero() -> Self {
        Self(0, 0)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}
