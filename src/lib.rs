#![no_std]
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

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut tty = tty::tty().lock();
    let (x, y) = tty.dimensions();
    let colors = [
        tty::vgatext::Color::Black,
        tty::vgatext::Color::Gray,
        tty::vgatext::Color::Blue,
        tty::vgatext::Color::LightBlue,
        tty::vgatext::Color::Green,
        tty::vgatext::Color::LightGreen,
        tty::vgatext::Color::Cyan,
        tty::vgatext::Color::LightCyan,
        tty::vgatext::Color::Red,
        tty::vgatext::Color::LightRed,
        tty::vgatext::Color::Magenta,
        tty::vgatext::Color::Pink,
        tty::vgatext::Color::Brown,
        tty::vgatext::Color::Yellow,
        tty::vgatext::Color::LightGray,
        tty::vgatext::Color::White,
    ];
    let mut cols = [tty::Character::blank(); 2000];

    for i in 0..x {
        for j in 0..y {
            cols[i + j * 80] =
                tty::Character::new(b' ', tty::TextColor::from_back(colors[i % colors.len()]));
        }
    }

    loop {
        cols.rotate_left(1);
        tty.sync_buff(&cols);
        tty.flush();
        for _ in 0..40_000_000 {
            unsafe { asm!("nop") }
        }
    }
}
