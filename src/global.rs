use core::ptr;

use crate::{
    console::{Console, noop},
    sync::spin::SpinLock,
};

pub static CONSOLE: SpinLock<Console> = SpinLock::new(Console {
    data: ptr::null_mut(),
    vtable: &noop::VTABLE,
});
