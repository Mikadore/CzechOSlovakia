#![no_std]
#![feature(asm)]
#![feature(prelude_import)]

pub mod memio;
pub mod tty;
pub mod util;
pub mod logging;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut i = 0;
    let _ = util::text::format_apply(
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
    logging::init().unwrap();
    log::info!("Started up kernel and initialized logging");
    tty::init();

    loop {
        unsafe {
            asm!("nop")
        }
    }
}
