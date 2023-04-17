use std::cell::{Cell, RefCell};

use crate::{DriverInfo, DriverShareRef, RwAllowBuffer};
use core::cmp;
use libtock_platform::ErrorCode;

pub struct Rng {
    buffer: RefCell<RwAllowBuffer>,
    remaining: Cell<usize>,
    idx: Cell<usize>,
    getting_randomness: Cell<bool>,
    share_ref: DriverShareRef,
    random_numbers: Cell<Option<Vec<u8>>>,
}

impl Rng {
    pub fn new() -> std::rc::Rc<Rng> {
        std::rc::Rc::new(Rng {
            buffer: Default::default(),
            remaining: Cell::new(0),
            idx: Cell::new(0),
            getting_randomness: Cell::new(false),
            share_ref: Default::default(),
            random_numbers: Cell::new(None),
        })
    }

    pub fn add_bytes(&self, buf: &[u8]) {
        if !self.getting_randomness.get() {
            return;
        }

        // this would happen only if the buffer was changed
        if self.idx.get() > self.buffer.borrow().len() {
            self.idx.set(0);
            self.remaining.set(0);
        } else {
            if self.idx.get() + self.remaining.get() > self.buffer.borrow().len() {
                self.remaining
                    .set(self.buffer.borrow().len() - self.idx.get())
            }

            let bytes_to_add = cmp::min(self.remaining.get(), buf.len());
            self.buffer.borrow_mut()[self.idx.get()..self.idx.get() + bytes_to_add]
                .copy_from_slice(&buf[..bytes_to_add]);
            self.remaining.set(self.remaining.get() - bytes_to_add);
            self.idx.set(self.idx.get() + bytes_to_add);
        }

        if self.remaining.get() == 0 {
            self.share_ref
                .schedule_upcall(0, (0, self.idx.get() as u32, 0))
                .expect("Unable to schedule upcall");
            self.getting_randomness.set(false);
            self.random_numbers.set(None);
        }
    }

    pub fn add_bytes_sync(&self, buf: &[u8]) {
        self.random_numbers.set(Some(Vec::from(buf)))
    }
}

impl crate::fake::SyscallDriver for Rng {
    fn info(&self) -> DriverInfo {
        DriverInfo::new(DRIVER_NUM).upcall_count(1)
    }

    fn register(&self, share_ref: DriverShareRef) {
        self.share_ref.replace(share_ref);
    }

    fn allow_readwrite(
        &self,
        buffer_num: u32,
        buffer: RwAllowBuffer,
    ) -> Result<RwAllowBuffer, (RwAllowBuffer, libtock_platform::ErrorCode)> {
        if buffer_num == RW_ALLOW {
            Ok(self.buffer.replace(buffer))
        } else {
            Err((buffer, ErrorCode::Invalid))
        }
    }

    fn command(&self, command_id: u32, data: u32, _: u32) -> libtock_platform::CommandReturn {
        match command_id {
            EXISTS => crate::command_return::success(),

            ASK_FOR_RANDOM_BYTES => {
                self.remaining.set(data as usize);
                self.idx.set(0);
                self.getting_randomness.set(true);

                if let Some(numbers) = self.random_numbers.take() {
                    self.add_bytes(&numbers);
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

const DRIVER_NUM: u32 = 0x40001;

// Command IDs

const EXISTS: u32 = 0;
const ASK_FOR_RANDOM_BYTES: u32 = 1;

const RW_ALLOW: u32 = 0;
