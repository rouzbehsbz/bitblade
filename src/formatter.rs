#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    fn get_foreground_ansi_escape_code(&self) -> u8 {
        match *self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
        }
    }

    fn get_background_ansi_escape_code(&self) -> u8 {
        match *self {
            Self::Black => 40,
            Self::Red => 41,
            Self::Green => 42,
            Self::Yellow => 43,
            Self::Blue => 44,
            Self::Magenta => 45,
            Self::Cyan => 46,
            Self::White => 47,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Style {
    Normal,
    Bold,
}

impl Style {
    fn get_ansi_escape_code(&self) -> u8 {
        match *self {
            Self::Normal => 22,
            Self::Bold => 1,
        }
    }
}
