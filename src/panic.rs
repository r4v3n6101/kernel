use core::{fmt::Write, panic::PanicInfo};

use crate::global;

/// Panic is copying the loggger logic to prevent a loop in case log's write gives an error
/// (probably caused by incorrect args formatting)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let console = &mut *global::CONSOLE.lock();

    // Skip an error to prevent panic loop
    let _ = if let Some(location) = info.location() {
        writeln!(
            console,
            "kernel panic at {} line {}:",
            location.file(),
            location.line(),
        )
    } else {
        writeln!(console, "kernel panic somewhere:")
    };
    let _ = writeln!(console, "{}", info.message());

    // TODO : halt
    loop {}
}
