pub mod vgatext;

pub use vgatext::Character;
pub use vgatext::Color;
pub use vgatext::TextColor;

/// The TTY is the interface the kernel uses to interact with a text screen.
/// The TTY, while being able to set characters randomly,
/// Keeps track of where the last character was written by appending functions, such as
/// putchar, cputchar, cputstr, putstr... (these are documented as appending),
/// and sequentially appends all text written after that.
/// It essentially mimics a normal console in that regards.
/// Once the TTY is full, it acts as a FIFO, discarding the beginning bytes.
pub struct TTY {
    pos: usize,
    col: TextColor,
    buff: [Character; 2000],
}

impl TTY {
    /// Character buffer width of the TTY
    pub fn width(&self) -> usize {
        vgatext::WIDTH
    }

    /// Character buffer height of the TTY
    pub fn height(&self) -> usize {
        vgatext::HEIGHT
    }

    /// The default color used for printing characters
    pub fn color(&self) -> TextColor {
        self.col
    }

    /// Sets the default color to be used for printing
    pub fn set_color(&mut self, col: TextColor) {
        self.col = col;
    }

    /// Returns the X coordinate of the NEXT character to be written
    pub fn x(&self) -> usize {
        self.pos % vgatext::WIDTH
    }

    /// Returns the Y coordinate of the NEXT character to be written
    pub fn y(&self) -> usize {
        self.pos / vgatext::WIDTH
    }

    /// Append-Writes a colored Character
    pub fn cputchar(&mut self, c: Character) {
        if c.ascii() == b'\n' {
            if self.y() == vgatext::HEIGHT - 1 {
                self.buff.rotate_left(vgatext::WIDTH);
                self.buff[(vgatext::HEIGHT - 1) * vgatext::WIDTH..].fill(Character::blank());
                self.pos = (vgatext::HEIGHT - 1) * vgatext::WIDTH ;
            } else {
                self.pos = (self.y() + 1) * vgatext::WIDTH;
            }
        } else if self.pos == self.buff.len() {
            self.buff.rotate_left(1);
            self.buff[self.buff.len() - 1] = c;
        } else {
            self.buff[self.pos] = c;
            self.pos += 1;
        }
    }

    /// Append writes an ascii character using the default color
    pub fn putchar(&mut self, c: u8) {
        self.cputchar(Character::new(c, self.col));
    }
    
    /// Append-Writes a colored character string
    pub fn cputstr(&mut self, str: &[Character]) {
        // TODO: Optimize
        for &c in str {
            self.cputchar(c)
        }
        self.flush()
    }

    /// Append-Writes an ascii string using the default character
    pub fn putstr(&mut self, str: &[u8]) {
        // TODO: Optimize
        for &c in str {
            self.putchar(c)
        }
        self.flush()
    }

    /// Writes a character to the screen. It is immediately visible - YOU DON'T NEED TO FLUSH
    pub fn put(&mut self, pos: (usize, usize), c: Character) {
        self.buff[pos.0 + pos.1*vgatext::WIDTH] = c;
        vgatext::writechar(pos, c);
    }

    /// Get the buffered character, NOT necessarily the currently displayed one
    pub fn get(&mut self, pos: (usize, usize)) -> Character {
        self.buff[pos.0 + pos.1*vgatext::WIDTH]
    }

    /// Clears the entire screen with the given character and flushes it afterwards
    /// # Example
    /// ```rust
    /// tty.clear(tty::Character::blank())
    /// ```
    pub fn clear(&mut self, clear_char: Character) {
        self.buff.fill(clear_char);
        self.flush();
        self.pos = 0;
    }

    /// Writes the character buffer to the actual video memory
    pub fn flush(&self) {
        vgatext::write_at((0, 0), &self.buff);
    }
}

impl core::fmt::Write for TTY {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if s.is_ascii() {
            self.putstr(s.as_bytes());
        } else {
            panic!("{}", "write_str({string}): Writing non ascii string");
        }
        Ok(())
    }
}

lazy_static::lazy_static!(
    /// Thread safe, static handle to the TTY 
    static ref TTY_INSTANCE: spin::Mutex<TTY> = spin::Mutex::<TTY>::from(TTY{
        pos: 0,
        col: TextColor::default(),
        buff: [Character::blank();2000]
    });
);

pub fn tty() -> &'static spin::Mutex<TTY> {
    &*TTY_INSTANCE
}

/// Mimics the `print!` macro, but acts on the TTY
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::tty::tty().lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

/// Mimics the `println!` macro, but acts on the TTY
#[macro_export]
macro_rules! kprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}