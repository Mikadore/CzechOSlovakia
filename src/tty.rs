pub mod vgatext;

pub use vgatext::Character;
pub use vgatext::Color;
pub use vgatext::TextColor;

pub struct TTY {
    pos: usize,
    col: Color,
    buff: [Character; 2000],
}

impl TTY {
    pub fn set_color(&mut self, col: Color) {
        self.col = col;
    }

    pub fn x(&self) -> usize {
        self.pos % vgatext::WIDTH
    }

    pub fn y(&self) -> usize {
        self.pos / vgatext::WIDTH
    }

    pub fn cputchar(&mut self, c: Character) {
        if c.ascii() == b'\n' {
            if self.y() == vgatext::HEIGHT - 1 {
                self.buff.rotate_left(vgatext::WIDTH);
                self.buff[(vgatext::HEIGHT - 1) * vgatext::WIDTH..].fill(Character::blank());
                self.buff[(vgatext::HEIGHT - 1) * vgatext::WIDTH] = c;
                self.pos = (vgatext::HEIGHT - 1) * vgatext::WIDTH + 1;
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

    pub fn putchar(&mut self, c: u8) {
        self.cputchar(Character::new(c, self.col));
    }

    pub fn cputstr(&mut self, str: &[Character]) {
        // TODO: Optimize
        for &c in str {
            self.cputchar(c)
        }
        self.flush()
    }

    pub fn putstr(&mut self, str: &[u8]) {
        // TODO: Optimize
        for &c in str {
            self.putchar(c)
        }
        self.flush()
    }

    pub fn clear(&mut self) {
        self.buff.fill(Character::blank());
        self.flush();
        self.pos = 0;
    }

    pub fn flush(&self) {
        vgatext::write_at((0, 0), &self.buff);
    }
}

impl core::fmt::Write for TTY {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if s.is_ascii() {
            self.putstr(s.as_bytes());
        } else {
            panic!("write_str({string}): Writing non ascii string");
        }
        Ok(())
    }
}

lazy_static::lazy_static!(
    pub static ref SCREEN: spin::Mutex<TTY> = spin::Mutex::<TTY>::from(TTY{
        pos: 0,
        col: Color::default(),
        buff: [Character::blank();2000]
    });
);



#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    // cannot fail
    let _ = SCREEN.lock().write_fmt(args);
}