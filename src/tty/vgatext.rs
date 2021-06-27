use crate::prelude::*;

#[repr(u8)]
pub enum TextColor {
    Black = 0x0,
    Gray = 0x8,
    Blue = 0x1,
    LightBlue = 0x9,
    Green = 0x2,
    LightGreen = 0xa,
    Cyan = 0x3,
    LightCyan = 0xb,
    Red = 0x4,
    LightRed = 0xc,
    Magenta = 0x5,
    Pink = 0xd,
    Brown = 0x6,
    Yellow = 0xe,
    LightGray = 0x7,
    White = 0xf,
}

impl From<u8> for TextColor {
    fn from(c: u8) -> Self {
        match c {
            0x0 => TextColor::Black,
            0x8 => TextColor::Gray,
            0x1 => TextColor::Blue,
            0x9 => TextColor::LightBlue,
            0x2 => TextColor::Green,
            0xa => TextColor::LightGreen,
            0x3 => TextColor::Cyan,
            0xb => TextColor::LightCyan,
            0x4 => TextColor::Red,
            0xc => TextColor::LightRed,
            0x5 => TextColor::Magenta,
            0xd => TextColor::Pink,
            0x6 => TextColor::Brown,
            0xe => TextColor::Yellow,
            0x7 => TextColor::LightGray,
            0xf => TextColor::White,
            _ => panic!("Bad conversion to TextColor from {}", c),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Color(u8);

impl Color {
    pub fn new(fore: TextColor, back: TextColor) -> Color {
        Color((back as u8) << 4 | (fore as u8))
    }

    pub fn default() -> Color {
        Self::new(TextColor::White, TextColor::Black)
    }

    pub fn from_fore(c: TextColor) -> Color {
        Self::new(c, TextColor::White)
    }

    pub fn from_back(c: TextColor) -> Color {
        Self::new(TextColor::White, c)
    }

    pub fn fore(&self) -> TextColor {
        TextColor::from(self.0)
    }

    pub fn back(&self) -> TextColor {
        TextColor::from(self.0 >> 4)
    }
}

impl From<Color> for u8 {
    fn from(b: Color) -> Self {
        b.0
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Character {
    ascii: u8,
    color: Color,
}

impl Character {
    pub fn new(ascii: u8, color: Color) -> Character {
        Character { ascii, color }
    }

    pub fn from_ascii(ascii: u8) -> Character {
        Character {
            ascii,
            color: Color::new(TextColor::White, TextColor::Black),
        }
    }

    pub fn blank() -> Character {
        Character {
            ascii: b' ',
            color: Color::new(TextColor::Black, TextColor::Black),
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ascii(&self) -> u8 {
        self.ascii
    }
}

impl From<u8> for Character {
    fn from(c: u8) -> Self {
        Character::from_ascii(c)
    }
}

pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;

pub fn write_at(pos: (usize, usize), src: &[Character]) {
    if pos.0 >= WIDTH
        || pos.1 >= HEIGHT
        || pos.0 + pos.1 * WIDTH + (src.len() - 1) >= WIDTH * HEIGHT
    {
        panic!(
            "write_at(({},{}), {{{}}}): invalid character location(s)",
            pos.0,
            pos.1,
            src.len()
        );
    }
    unsafe {
        crate::memio::vmemwrite(
            (0xb8000 + (pos.0 + pos.1 * WIDTH) * 2) as u64,
            crate::memio::cast_slice(src),
            src.len() * 2,
        )
    }
}

pub fn write_default_at(pos: (usize, usize), src: &[u8], color: Color) {
    if pos.0 >= WIDTH
        || pos.1 >= HEIGHT
        || pos.0 + pos.1 * WIDTH + (src.len() - 1) >= WIDTH * HEIGHT
    {
        panic!(
            "write_at(({},{}), {{{}}}, {{color}}): invalid character location(s)",
            pos.0,
            pos.1,
            src.len()
        );
    }
    let color = color.into();
    let baseaddr = (0xb8000 + (pos.0 + pos.1 * WIDTH) * 2) as *mut u8;

    unsafe {
        for (i, &b) in src.iter().enumerate() {
            baseaddr.add(2 * i).write_volatile(b);
            baseaddr.add(2 * i + 1).write_volatile(color);
        }
    }
}

pub fn writechar(pos: (usize, usize), char: Character) {
    if pos.0 >= WIDTH || pos.1 >= HEIGHT {
        panic!("writechar(({},{}), {{character}})", pos.0, pos.1)
    }
    unsafe {
        memio::vwrite((0xb8000 + (pos.0 + pos.1 * WIDTH) * 2) as u64, char.ascii());
        memio::vwrite(
            (0xb8001 + (pos.0 + pos.1 * WIDTH) * 2) as u64,
            char.color().into(),
        );
    }
}
