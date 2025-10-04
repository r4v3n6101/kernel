use crate::console::ConsoleVTable;

pub static VTABLE: ConsoleVTable = ConsoleVTable { put: |_, _| {} };
