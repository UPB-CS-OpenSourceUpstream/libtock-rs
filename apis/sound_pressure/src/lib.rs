#![no_std]

use core::cell::Cell;
use libtock_platform::{
    share, subscribe::OneId, DefaultConfig, ErrorCode, Subscribe, Syscalls, Upcall,
};

pub struct SoundPressure<S: Syscalls>(S);

impl<S: Syscalls> SoundPressure<S> {
    /// Returns Ok() if the driver was present.This does not necessarily mean
    /// that the driver is working.
    pub fn exists() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, EXISTS, 0, 0).to_result()
    }

    /// Initiate a SoundPressure measurement.
    ///
    /// This function is used both for synchronous and asynchronous readings
    pub fn read_sound_pressure() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, READ_TEMP, 0, 0).to_result()
    }

    /// Register an events listener
    pub fn register_listener<'share, F: Fn(i32)>(
        listener: &'share SoundPressureListener<F>,
        subscribe: share::Handle<Subscribe<'share, S, DRIVER_NUM, 0>>,
    ) -> Result<(), ErrorCode> {
        S::subscribe::<_, _, DefaultConfig, DRIVER_NUM, 0>(subscribe, listener)
    }

    /// Unregister the events listener
    pub fn unregister_listener() {
        S::unsubscribe(DRIVER_NUM, 0)
    }

    /// Initiate a synchronous SoundPressure measurement.
    /// Returns Ok(SoundPressure_value) if the operation was successful
    /// SoundPressure_value is returned in hundreds of centigrades
    pub fn read_sound_pressure_sync() -> Result<i32, ErrorCode> {
        let SoundPressure_cell: Cell<Option<i32>> = Cell::new(None);
        let listener = SoundPressureListener(|temp_val| {
            SoundPressure_cell.set(Some(temp_val));
        });
        share::scope(|subscribe| {
            if let Ok(()) = Self::register_listener(&listener, subscribe) {
                if let Ok(()) = Self::read_sound_pressure() {
                    while SoundPressure_cell.get() == None {
                        S::yield_wait();
                    }
                }
            }
        });

        match SoundPressure_cell.get() {
            None => Err(ErrorCode::Busy),
            Some(temp_val) => Ok(temp_val),
        }
    }
}

pub struct SoundPressureListener<F: Fn(i32)>(pub F);
impl<F: Fn(i32)> Upcall<OneId<DRIVER_NUM, 0>> for SoundPressureListener<F> {
    fn upcall(&self, temp_val: u32, _arg1: u32, _arg2: u32) {
        self.0(temp_val as i32)
    }
}

#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x60006;

// Command IDs

const EXISTS: u32 = 0;
const READ_TEMP: u32 = 1;
