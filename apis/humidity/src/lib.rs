#![no_std]

use core::cell::Cell;
use libtock_platform::{share, DefaultConfig, ErrorCode, Subscribe, Syscalls};

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
    pub fn register_listener<'share>(
        listener: &'share Cell<Option<(u32,)>>,
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
        let listener: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope(|subscribe| {
            Self::register_listener(&listener, subscribe)?;
            Self::read()?;
            while listener.get() == None {
                S::yield_wait();
            }

            match listener.get() {
                None => Err(ErrorCode::Busy),
                Some(hum_val) => Ok(hum_val.0),
            }
        })
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
