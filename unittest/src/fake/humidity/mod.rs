//! Fake implementation of the Humidity API, documented here:
//! https://github.com/tock/tock/blob/master/doc/syscalls/60001_humidity.md
//!
//! Like the real API, `Humidity` controls a fake humidity sensor. It provides
//! a function `set_value` used to immediately call an upcall with a humidity value read by the sensor
//! and a function 'set_value_sync' used to call the upcall when the read command is received.

use crate::{DriverInfo, DriverShareRef};
use libtock_platform::{CommandReturn, ErrorCode};
use std::cell::Cell;

// The `upcall_on_command` field is set to Some(value) if an upcall(with value as its argument) should be called when read command is received,
// or None otherwise. It was needed for testing `read_sync` library function which simulates a synchronous humidity read,
// because it was impossible to schedule an upcall during the `synchronous` read in other ways.
pub struct Humidity {
    busy: Cell<bool>,
    upcall_on_command: Cell<Option<i32>>,
    share_ref: DriverShareRef,
}

impl Humidity {
    pub fn new() -> std::rc::Rc<Humidity> {
        std::rc::Rc::new(Humidity {
            busy: Cell::new(false),
            upcall_on_command: Cell::new(None),
            share_ref: Default::default(),
        })
    }

    pub fn is_busy(&self) -> bool {
        self.busy.get()
    }
    pub fn set_value(&self, value: i32) {
        if self.busy.get() {
            self.share_ref
                .schedule_upcall(0, (value as u32, 0, 0))
                .expect("Unable to schedule upcall");
            self.busy.set(false);
        }
    }
    pub fn set_value_sync(&self, value: i32) {
        self.upcall_on_command.set(Some(value));
    }
}

impl crate::fake::SyscallDriver for Humidity {
    fn info(&self) -> DriverInfo {
        DriverInfo::new(DRIVER_NUM).upcall_count(1)
    }

    fn register(&self, share_ref: DriverShareRef) {
        self.share_ref.replace(share_ref);
    }

    fn command(&self, command_id: u32, _argument0: u32, _argument1: u32) -> CommandReturn {
        match command_id {
            EXISTS => crate::command_return::success(),

            READ_HUM => {
                if self.busy.get() {
                    return crate::command_return::failure(ErrorCode::Busy);
                }
                self.busy.set(true);
                if let Some(val) = self.upcall_on_command.take() {
                    self.set_value(val);
                }
                crate::command_return::success()
            }
            _ => crate::command_return::failure(ErrorCode::NoSupport),
        }
    }
}

#[cfg(test)]
mod tests;
// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x60001;

// Command IDs

const EXISTS: u32 = 0;
const READ_HUM: u32 = 1;
