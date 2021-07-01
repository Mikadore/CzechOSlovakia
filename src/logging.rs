/// Logs to serial port, mostly for QEMU
struct SerialLogger;

impl log::Log for SerialLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let _ = crate::util::text::format_apply(
                |s| {
                    for &b in s.as_bytes() {
                        unsafe { crate::memio::outb(0x3F8, b) }
                    } 
                    Ok(())
                },
                format_args!(
                    "[{}]@{}:{}> {}\n",
                    record.level(),
                    record.file().unwrap_or("none"),
                    record.line().unwrap_or(0),
                    record.args()
                ),
            );
        }
    }

    fn flush(&self) {}
}
static LOGGER: SerialLogger = SerialLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|_| log::set_max_level(log::LevelFilter::Debug))
}
