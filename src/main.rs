#![no_std]
#![no_main]
#![feature(asm)]

pub mod memio;
pub mod prelude;
pub mod tty;
pub mod util;
pub use prelude::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut i = 0;
    let _ = tty::format_apply(
        |s| {
            for ch in s.as_bytes() {
                if i == tty::vgatext::HEIGHT * tty::vgatext::WIDTH - 1 {
                    #[allow(clippy::empty_loop)]
                    loop {}
                }
                unsafe {
                    tty::vgatext::writechar(
                        (i % tty::vgatext::WIDTH, i / tty::vgatext::WIDTH),
                        tty::vgatext::Character::from_ascii(*ch),
                    )
                };
                i += 1;
            }
            Ok(())
        },
        format_args!("{}", info),
    );

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {

    #[allow(clippy::empty_loop)]
    loop {}
}
