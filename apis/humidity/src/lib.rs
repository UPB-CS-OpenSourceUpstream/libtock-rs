#![no_std]

use core::cell::Cell;
use libtock_platform::{
    share, subscribe::OneId, DefaultConfig, ErrorCode, Subscribe, Syscalls, Upcall,
};

pub struct Humidity<S: Syscalls>(S);

impl<S: Syscalls> Humidity<S> {
    /// Returns Ok() if the driver was present.This does not necessarily mean
    /// that the driver is working.
    pub fn exists() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, EXISTS, 0, 0).to_result()
    }

    /// Initiate a humidity measurement.
    ///
    /// This function is used both for synchronous and asynchronous readings
    pub fn read() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, READ_HUM, 0, 0).to_result()
    }

    /// Register an events listener
    pub fn register_listener<'share, F: Fn(u32)>(
        listener: &'share HumidityListener<F>,
        subscribe: share::Handle<Subscribe<'share, S, DRIVER_NUM, 0>>,
    ) -> Result<(), ErrorCode> {
        S::subscribe::<_, _, DefaultConfig, DRIVER_NUM, 0>(subscribe, listener)
    }

    /// Unregister the events listener
    pub fn unregister_listener() {
        S::unsubscribe(DRIVER_NUM, 0)
    }

    /// Initiate a synchronous humidity measurement.
    /// Returns Ok(humidity_value) if the operation was successful
    /// humidity_value is returned in hundreds of percent
    pub fn read_sync() -> Result<u32, ErrorCode> {
        let humidity_cell: Cell<Option<u32>> = Cell::new(None);
        let listener = HumidityListener(|humidity_val| {
            humidity_cell.set(Some(humidity_val));
        });
        share::scope(|subscribe| {
            Self::register_listener(&listener, subscribe)?;
            Self::read()?;
            while humidity_cell.get() == None {
                S::yield_wait();
            }

            match humidity_cell.get() {
                None => Err(ErrorCode::Busy),
                Some(humidity_val) => Ok(humidity_val),
            }
        })
    }
}

/// A wrapper around a closure to be registered and called when
/// a humidity reading is done.
///
/// ```ignore
/// let listener = HumidityListener(|humidity_val| {
///     // make use of the humidity value
/// });
/// ```
pub struct HumidityListener<F: Fn(u32)>(pub F);

impl<F: Fn(u32)> Upcall<OneId<DRIVER_NUM, 0>> for HumidityListener<F> {
    fn upcall(&self, humidity: u32, _arg1: u32, _arg2: u32) {
        self.0(humidity)
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
