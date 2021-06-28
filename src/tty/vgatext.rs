use crate::prelude::*;

/// VGA 4 Bit Colors
#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
            _ => panic!("Color::from({}): Bad conversion to Color", c),
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

pub fn vga_init() {
    unsafe {
        // disable cursor
        memio::mmio_outb(0x3D4, 0xA);
        memio::mmio_outb(0x3D5, 0x20); 

        // disable blinking
        memio::mmio_inb(0x3DA);
        memio::mmio_outb(0x3C0, 0x30);
        let state = memio::mmio_inb(0x3C1);
        memio::mmio_outb(0x3C0,  state & 0xF7);
    }
}

/// Write a slice of characters, starting from a specific character.
/// # Safety
/// Validate that the position is valid, and that the characters fit
pub unsafe fn write_at(pos: (usize, usize), src: &[Character]) {
    crate::memio::vmemwrite(
        (0xb8000 + (pos.0 + pos.1 * WIDTH) * 2) as u64,
        crate::memio::cast_slice(src),
        src.len() * 2,
    )
}

/// Write a slice of ascii characters, all of the same specified color,
/// starting at a specific character.
/// # Safety
/// Validate that the position is in bounds, and that the string fits
pub unsafe fn write_color_at(pos: (usize, usize), src: &[u8], color: TextColor) {
    let color = color.into();
    let baseaddr = (0xb8000 + (pos.0 + pos.1 * WIDTH) * 2) as *mut u8;

    for (i, &b) in src.iter().enumerate() {
        baseaddr.add(2 * i).write_volatile(b);
        baseaddr.add(2 * i + 1).write_volatile(color);
    }
}

/// Write a single character to a position.
/// # Safety
/// Validate that he position is valid
pub unsafe fn writechar(pos: (usize, usize), char: Character) {
    memio::vwrite((0xb8000 + (pos.0 + pos.1 * WIDTH) * 2) as u64, &char);
}

/// Reset the video memory
pub fn reset() {
    unsafe {
        memio::vmemset(0xb8000, 0, WIDTH * HEIGHT * 2);
    }
}

pub fn blink() -> bool {
    let mut out: u64;
    unsafe {
        asm!("
            mov dx, 0x03DA
            in al, dx
            mov dx, 0x03C0
            mov al, 0x30
            out dx, al
            inc dx
            in al, dx
            and al, 0xF7
            jz .VGA_BLINK_ON
            mov {0}, 0
        .VGA_BLINK_ON:
            mov {0}, 1
            ", out(reg) out
        );
    };
    out == 1
}
