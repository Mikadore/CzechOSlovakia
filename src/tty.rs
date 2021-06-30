pub mod vgatext;

pub use vgatext::Character;
pub use vgatext::Color;
pub use vgatext::TextColor;

/// The TTY is the interface the kernel uses to interact with a text screen. [^newline]
/// If you want a terminal like interface, use the `kprint!` or `kprintln!`
/// macros for formatted input and automated flushing[^macros].
/// You can also use the `append_*` functions which append after the last written character.
/// You can retreive the position of the *next* character to be written character via `continue_pos`
/// (you may also set it via `set_pos`).
///
/// [^newline]: Newlines don't work. The full character map is called CP437, the
///     code 10 (\n) is displayed as ◙
///
/// [^macros]: DO NOT use these while holding the TTY lock, you *will* deadlock.
/// ```rust
/// let (x, y) = {
/// {
///     let tty = tty::tty().lock();
///     tty.dimensions()
/// }
/// };
/// let array = [
///     tty::Color::Red,
///     tty::Color::LightRed,
///     tty::Color::Yellow,
///     tty::Color::Green,
///     tty::Color::Blue,
/// ];
/// {
///     let mut tty = tty::tty().lock();
///     let (width, _) = tty.dimensions();
///     for y in 0..y {
///         for x in 0..x {
///             tty.cput((x,y), tty::Character::new(b' ', tty::TextColor::from_back(array[x/(width/array.len())])))
///         }
///     }
///     tty.set_pos((19,12));
///     tty.flush()
/// }
///
///
///
/// kprint!("VGA Display: {}x{} Characters with: ", x, y);
/// let mut tty = tty::tty().lock();
/// tty.append_char(tty::Character::new(
///     b'C',
///     tty::TextColor::from_back(tty::Color::Red),
/// ));
/// tty.append_char(tty::Character::new(
///     b'O',
///     tty::TextColor::from_back(tty::Color::LightRed),
/// ));
/// tty.append_char(tty::Character::new(
///     b'L',
///     tty::TextColor::from_back(tty::Color::Yellow),
/// ));
/// tty.append_char(tty::Character::new(
///     b'O',
///     tty::TextColor::from_back(tty::Color::Green),
/// ));
/// tty.append_char(tty::Character::new(
///     b'R',
///     tty::TextColor::from_back(tty::Color::Blue),
/// ));
/// tty.append_char(tty::Character::new(
///     b'S',
///     tty::TextColor::from_back(tty::Color::Magenta),
/// ));
/// tty.flush();
/// ```
pub struct TTY {
    /// tracks position for the ktty* macros
    pos: usize,
    col: TextColor,
    buff: [Character; 2000],
    /// if the tty is a copy, any accesses to vram are ignored
    is_copy: bool,
}

impl TTY {
    /// Creates a TTY, which DOES NOT sync with the screen.
    /// It's used to `tty.sync(tty_copy)` with the actually tty
    #[allow(clippy::new_without_default)]
    pub fn new() -> TTY {
        TTY {
            pos: 0,
            col: TextColor::default(),
            buff: [Character::blank(); 2000],
            is_copy: true,
        }
    }

    /// Dimensions (in characters) of the TTY
    /// # Example
    /// ```rust
    /// let (width, height) = tty.dimensions();
    /// ```
    pub fn dimensions(&self) -> (usize, usize) {
        (vgatext::WIDTH, vgatext::HEIGHT)
    }

    /// The default color used for printing characters
    pub fn color(&self) -> TextColor {
        self.col
    }

    /// Sets the default color to be used for printing
    /// This color's background color will be used for clearing as well
    pub fn set_color(&mut self, col: TextColor) -> &mut Self {
        self.col = col;
        self
    }

    /// Returns the next position the k* tty macros will print at
    pub fn continue_pos(&self) -> usize {
        self.pos
    }

    /// Overwrites the k* tty macro position, panics if pos is invalid
    pub fn set_pos(&mut self, pos: (usize, usize)) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!("set_pos(({},{})): invalid position", pos.0, pos.1);
        }
        self.pos = pos.0 + pos.1 * vgatext::WIDTH;
        self
    }

    /// A convenience to write multile colored characters at once.
    /// Works with newlines.
    pub fn cputstr(&mut self, pos: (usize, usize), str: &[Character]) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!(
                "putstr(({},{}), {{string}}): invalid position",
                pos.0, pos.1
            );
        } else if pos.0 + pos.1 * vgatext::WIDTH + str.len() >= vgatext::WIDTH * vgatext::HEIGHT {
            panic!(
                "putstr(({},{}), {{len: {}}}): string too big",
                pos.0,
                pos.1,
                str.len()
            );
        }
        let mut i = 0;
        let start = pos.0 + pos.1 * vgatext::WIDTH;
        for &b in str.iter() {
            if start + i == vgatext::WIDTH * vgatext::HEIGHT {
                panic!(
                    "putstr(({},{}), {{len: {}}}): string too big",
                    pos.0,
                    pos.1,
                    str.len()
                );
            } else if b.ascii() == b'\n' {
                i += 80 - (pos.0 + i) % 80;
            } else {
                self.buff[start + i] = b;
            }
            i += 1;
        }
        self
    }

    /// A conveniece to write multiple ascii characters at once.
    /// Works with newlines.
    pub fn putstr(&mut self, pos: (usize, usize), str: &[u8]) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!(
                "putstr(({},{}), {{string}}): invalid position",
                pos.0, pos.1
            );
        } else if pos.0 + pos.1 * vgatext::WIDTH + str.len() >= vgatext::WIDTH * vgatext::HEIGHT {
            panic!(
                "putstr(({},{}), {{len: {}}}): string too big",
                pos.0,
                pos.1,
                str.len()
            );
        }
        let mut i = 0;
        let start = pos.0 + pos.1 * vgatext::WIDTH;
        for &b in str.iter() {
            if start + i == vgatext::WIDTH * vgatext::HEIGHT {
                panic!(
                    "putstr(({},{}), {{len: {}}}): string too big",
                    pos.0,
                    pos.1,
                    str.len()
                );
            }
            if b == b'\n' {
                i += 80 - (pos.0 + i) % 80;
            } else {
                self.buff[start + i] = Character::new(b, self.col);
            }
            i += 1;
        }
        self
    }

    /// Writes a colored character to the screen. Flushes the (single) character
    pub fn cput(&mut self, pos: (usize, usize), c: Character) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!(
                "cput(({},{}), {{character}}): invalid position",
                pos.0, pos.1
            )
        }
        self.buff[pos.0 + pos.1 * vgatext::WIDTH] = c;
        self
    }

    /// Writes an ascii character to the screen. Flushes the (single) character
    pub fn put(&mut self, pos: (usize, usize), c: u8) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!(
                "put(({},{}), {{character}}): invalid position",
                pos.0, pos.1
            )
        }
        let c = Character::new(c, self.color());
        self.buff[pos.0 + pos.1 * vgatext::WIDTH] = c;
        self
    }

    /// Writes a colored character to the screen. Also writes directly to video memory.
    /// Use this instead of immediately flushing, as it is much cheaper
    pub fn cput_force(&mut self, pos: (usize, usize), c: Character) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!(
                "cput_force(({},{}), {{character}}): invalid position",
                pos.0, pos.1
            )
        }
        self.buff[pos.0 + pos.1 * vgatext::WIDTH] = c;
        if !self.is_copy {
            unsafe { vgatext::writechar(pos, c) }
        }
        self
    }

    /// Writes an ascii character to the screen. Also writes directly to video memory.
    /// Use this instead of flushing, as it is much cheaper
    pub fn put_force(&mut self, pos: (usize, usize), c: u8) -> &mut Self {
        if pos.0 >= vgatext::WIDTH || pos.1 >= vgatext::HEIGHT {
            panic!(
                "put_force(({},{}), {{character}}): invalid position",
                pos.0, pos.1
            )
        }
        let c = Character::new(c, self.color());
        self.buff[pos.0 + pos.1 * vgatext::WIDTH] = c;
        if !self.is_copy {
            unsafe { vgatext::writechar(pos, c) }
        }
        self
    }

    /// Get the buffered character, NOT necessarily the currently displayed one.
    pub fn get(&mut self, pos: (usize, usize)) -> Character {
        self.buff[pos.0 + pos.1 * vgatext::WIDTH]
    }

    /// Append-Writes a colored Character.
    /// This is mainly used by the k* tty macros.
    /// If the TTY is full, it acts as a FIFO, discarding beginning characters.
    pub fn append_char(&mut self, c: Character) -> &mut Self {
        if c.ascii() == b'\n' {
            if self.pos / vgatext::WIDTH == vgatext::HEIGHT - 1 {
                self.buff.rotate_left(vgatext::WIDTH);
                self.buff[(vgatext::HEIGHT - 1) * vgatext::WIDTH..].fill(Character::blank());
                self.pos = (vgatext::HEIGHT - 1) * vgatext::WIDTH;
            } else {
                self.pos = (self.pos / vgatext::WIDTH + 1) * vgatext::WIDTH;
            }
        } else if self.pos == self.buff.len() {
            self.buff.rotate_left(1);
            self.buff[self.buff.len() - 1] = c;
        } else {
            self.buff[self.pos] = c;
            self.pos += 1;
        }
        self
    }

    /// Append-Writes an ascii string
    /// If the TTY is full, it acts as a FIFO, discarding beginning characters.
    pub fn append_str(&mut self, c: &[u8]) -> &mut Self {
        // optimization for bigger than TTY strings
        // TODO: Optimize more. Rotate up to max(remaining, 80)s chars at newline
        let count =
            (c.len() / (vgatext::WIDTH * vgatext::HEIGHT)) * vgatext::HEIGHT * vgatext::WIDTH;
        if count == 0 {
            for &b in c {
                self.append_char(Character::new(b, self.col));
            }
        } else {
            for &b in &c[c.len() - (count * vgatext::WIDTH * vgatext::HEIGHT)..] {
                self.append_char(Character::new(b, self.col));
            }
        }
        self
    }

    /// Clears the entire screen with the given character and flushes it afterwards.
    /// # Example
    /// ```rust
    /// tty.clear(tty::Character::blank())
    /// ```
    pub fn clear_char(&mut self, clear_char: Character) -> &mut Self {
        self.buff.fill(clear_char);
        self.flush();
        self.pos = 0;
        self
    }

    /// Clears the entire screen using the default color's background
    pub fn clear(&mut self) -> &mut Self {
        self.clear_char(Character::new(b' ', self.col))
    }

    /// Resets the state of the TTY as it was at boot time
    /// Note: this does include clearing the screen
    pub fn reset(&mut self) -> &mut Self {
        self.pos = 0;
        self.col = TextColor::default();
        self.buff.fill(Character::blank());
        if !self.is_copy {
            vgatext::reset();
        }
        self
    }

    /// Writes the character buffer to the actual video memory
    pub fn flush(&mut self) -> &mut Self {
        if !self.is_copy {
            unsafe {
                vgatext::write_at((0, 0), &self.buff);
            }
        }
        self
    }

    /// Copies `other`s buffer. Does not flush the screen.
    pub fn sync(&mut self, other: &TTY) -> &mut Self {
        self.buff.copy_from_slice(&other.buff);
        self
    }

    /// Synchronizes internal buffer from `other`. Does not flush the screen
    pub fn sync_buff(&mut self, other: &[Character]) -> &mut Self {
        if other.len() != self.buff.len() {
            panic!(
                "TTY::sync_buff(0x{:x}, 0x{:x}): called with invalid size",
                self.buff.as_ptr() as u64,
                other.as_ptr() as u64
            );
        }
        self.buff.copy_from_slice(other);
        self
    }

    /// Access the character buffer
    pub fn buff(&self) -> &[Character] {
        &self.buff
    }

    /// Copies itself. New TTY Instance DOES NOT sync with Video RAM
    pub fn copy(&self) -> TTY {
        let mut tty = TTY::new();
        tty.buff.copy_from_slice(&self.buff);
        tty
    }
}

pub fn format_apply<F>(apply: F, args: core::fmt::Arguments<'_>) -> core::fmt::Result
where
    F: FnMut(&str) -> core::fmt::Result,
{
    struct FakeWriter<F: FnMut(&str) -> core::fmt::Result> {
        functor: F,
    }
    impl<F: FnMut(&str) -> core::fmt::Result> core::fmt::Write for FakeWriter<F> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            (self.functor)(s)
        }
    }
    use core::fmt::Write;
    FakeWriter { functor: apply }.write_fmt(args)
}

lazy_static::lazy_static!(
    /// Thread safe, static handle to the TTY
    static ref TTY_INSTANCE: spin::Mutex<TTY> = spin::Mutex::<TTY>::from((|| {
        vgatext::vga_init();
        TTY {
            pos: 0,
            col: TextColor::default(),
            buff: [Character::blank();2000],
            is_copy: false
        }
    })());
);

pub fn tty() -> &'static spin::Mutex<TTY> {
    &*TTY_INSTANCE
}

/// Mimics the `print!` macro, but acts on the TTY
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        let mut tty = $crate::tty::tty().lock();
        $crate::tty::format_apply(|s| {
            if s.is_ascii() {
                tty.append_str(s.as_bytes());
            } else {
                panic!("kprint!(...): formatted string contains non-ascii characters");
            }
            Ok(())
        }, format_args!($($arg)*)).unwrap();
        tty.flush();
    }};
}

/// Mimics the `println!` macro, but acts on the TTY
#[macro_export]
macro_rules! kprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {$crate::kprint!("{}\n", format_args!($($arg)*))};
}
