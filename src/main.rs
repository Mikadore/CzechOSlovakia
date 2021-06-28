#![no_std]
#![no_main]
#![feature(asm)]

pub mod memio;
pub mod prelude;
pub mod tty;
pub mod util;
pub use prelude::*;

use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut screen = tty::tty().lock();
    screen.clear(tty::Character::blank());
    let _ = write!(screen, "{}", info);

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let (x,y) = {
        let tty = tty::tty().lock();
        (tty.width(), tty.height())
    };
    kprint!("VGA Display: {}x{} Characters with: ", x, y);
    {
        let mut tty = tty::tty().lock();
        tty.cputchar(tty::Character::new(b'C', tty::TextColor::from_back(tty::Color::Red)));
        tty.cputchar(tty::Character::new(b'O', tty::TextColor::from_back(tty::Color::LightRed)));
        tty.cputchar(tty::Character::new(b'L', tty::TextColor::from_back(tty::Color::Yellow)));
        tty.cputchar(tty::Character::new(b'O', tty::TextColor::from_back(tty::Color::Green)));
        tty.cputchar(tty::Character::new(b'R', tty::TextColor::from_back(tty::Color::Blue)));
        tty.cputchar(tty::Character::new(b'S', tty::TextColor::from_back(tty::Color::Magenta)));
        tty.flush();
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
