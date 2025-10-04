use core::fmt::Write;

use crate::{console::Console, global};

/// Logger that feeds data into the global console
pub struct GlobalConsoleLogger;

impl log::Log for GlobalConsoleLogger {
    /// Always true, because kernel must see all the messages.
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        // It will lock for every line, however it may not be mandatory.
        // But it's just simplification, you may make your log with clever logic allowing one lock for the batch of lines.
        let mut console = *global::CONSOLE.lock() as &dyn Console;

        writeln!(console, "{} - {}", record.level(), record.args()).expect("fail to log line");
    }

    fn flush(&self) {}
}
