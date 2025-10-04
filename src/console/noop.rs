use super::Console;

pub struct Noop;

impl Console for Noop {
    fn put(&self, _: u8) {
        // no-op
    }
}
