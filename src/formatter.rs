#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub fn to_foreground_ansi_escape_code(&self) -> &str {
        match *self {
            Self::Black => "\x1B[30m",
            Self::Red => "\x1B[31m",
            Self::Green => "\x1B[32m",
            Self::Yellow => "\x1B[33m",
            Self::Blue => "\x1B[34m",
            Self::Magenta => "\x1B[35m",
            Self::Cyan => "\x1B[36m",
            Self::White => "\x1B[37m",
        }
    }

    pub fn to_background_ansi_escape_code(&self) -> &str {
        match *self {
            Self::Black => "\x1B[40m",
            Self::Red => "\x1B[41m",
            Self::Green => "\x1B[42m",
            Self::Yellow => "\x1B[43m",
            Self::Blue => "\x1B[44m",
            Self::Magenta => "\x1B[45m",
            Self::Cyan => "\x1B[46m",
            Self::White => "\x1B[47m",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Normal,
    Bold,
}

impl Style {
    pub fn to_ansi_escape_code(&self) -> &str {
        match *self {
            Self::Normal => "\x1B[22m",
            Self::Bold => "\x1B[1m",
        }
    }
}
