use crate::{
    console::{Console, noop::Noop as NoopConsole},
    sync::spin::SpinLock,
};

type GlobalConsole = &'static (dyn Console + Sync);

pub static CONSOLE: SpinLock<GlobalConsole> = SpinLock::new(&NoopConsole);
