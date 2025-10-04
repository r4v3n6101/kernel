use core::fmt::Write;

use crate::global;

/// Logger that feeds data into the global console
pub struct GlobalConsoleLogger;

impl log::Log for GlobalConsoleLogger {
    /// Always true, because kernel must see all the messages.
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        // It will lock for every line, however it may not be mandatory, but it's just simplification.
        // You may make your log with clever logic allowing one lock for the batch of lines.
        let console = &mut *global::CONSOLE.lock();
        writeln!(console, "{} - {}", record.level(), record.args()).expect("fail to log line");
    }

    fn flush(&self) {}
}
