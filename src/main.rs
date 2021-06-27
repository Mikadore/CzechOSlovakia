#![no_std]
#![no_main]

pub mod memio;
pub mod prelude;
pub mod tty;
pub mod util;
pub use prelude::*;

use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut screen = tty::SCREEN.lock();
    screen.clear();
    let _ = write!(screen, "{}", info);

    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kprintln!("Hello {}", "World!");

    #[allow(clippy::empty_loop)]
    loop {}
}
