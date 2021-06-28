use crate::prelude::*;

/// VGA 4 Bit Colors
#[repr(u8)]
pub enum Color {
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

impl From<u8> for Color {
    fn from(c: u8) -> Self {
        match c {
            0x0 => Color::Black,
            0x8 => Color::Gray,
            0x1 => Color::Blue,
            0x9 => Color::LightBlue,
            0x2 => Color::Green,
            0xa => Color::LightGreen,
            0x3 => Color::Cyan,
            0xb => Color::LightCyan,
            0x4 => Color::Red,
            0xc => Color::LightRed,
            0x5 => Color::Magenta,
            0xd => Color::Pink,
            0x6 => Color::Brown,
            0xe => Color::Yellow,
            0x7 => Color::LightGray,
            0xf => Color::White,
            _ => panic!("Bad conversion to Color from {}", c),
        }
    }
}

/// VGA Color Point
/// Consists of a 4 bit VGA foreground and background color
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TextColor(u8);

impl TextColor {
    /// Constructs a new TextColor from a fore and background TextColor
    pub fn new(fore: Color, back: Color) -> TextColor {
        TextColor((back as u8) << 4 | (fore as u8))
    }

    /// The default TTY TextColor, white on black
    pub fn default() -> TextColor {
        Self::new(Color::White, Color::Black)
    }

    /// Constructs a new TextColor from a foreground, the background is black
    pub fn from_fore(c: Color) -> TextColor {
        Self::new(c, Color::Black)
    }

    /// Constructs a new Color from a background, the foreground is white
    pub fn from_back(c: Color) -> TextColor {
        Self::new(Color::White, c)
    }

    /// The foreground of the Color
    pub fn fore(&self) -> Color {
        Color::from(self.0)
    }

    /// The background of the Color
    pub fn back(&self) -> Color {
        Color::from(self.0 >> 4)
    }
}

impl From<TextColor> for u8 {
    fn from(b: TextColor) -> Self {
        b.0
    }
}

/// VGA Character Point
/// Consists of an ASCII (Code Page 437) character and a Color
/// The layout is the same as the physical code point in VGA Ram
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Character {
    ascii: u8,
    color: TextColor,
}

impl Character {
    /// Constructs a new VGA Character from a Color and the ASCII character
    pub fn new(ascii: u8, color: TextColor) -> Character {
        Character { ascii, color }
    }

    /// Constructs a new VGA Character from an ASCII character
    /// The Color is left as the default Color, i.e. white on black 
    pub fn from_ascii(ascii: u8) -> Character {
        Character {
            ascii,
            color: TextColor::default(),
        }
    }

    /// The blank character.
    /// A black space character.
    pub fn blank() -> Character {
        Character {
            ascii: b' ',
            color: TextColor::new(Color::Black, Color::Black),
        }
    }

    /// The Color
    pub fn color(&self) -> TextColor {
        self.color
    }

    /// The ascii character
    pub fn ascii(&self) -> u8 {
        self.ascii
    }
}

impl From<u8> for Character {
    fn from(c: u8) -> Self {
        Character::from_ascii(c)
    }
}

/// The VGA display buffer height
pub const HEIGHT: usize = 25;
/// The VGA display buffer width
pub const WIDTH: usize = 80;

/// Write a slice of characters, starting from a specific character.
/// Panics if the pos is invalid, or the characters would go
/// out of bounds.
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

/// Write a slice of ascii characters, all of the same specified color,
/// starting at a specific character. Panics if the pos is invalid,
/// or the characters would go out of bounds
pub fn write_color_at(pos: (usize, usize), src: &[u8], color: TextColor) {
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

/// Write a single character to a position. Panics if said position is invalid
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
