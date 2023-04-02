#![no_std]

//use core::cell::Cell;
use libtock_platform::{
    share, subscribe::OneId, CommandReturn, DefaultConfig, ErrorCode, Subscribe, Syscalls, Upcall,
};

pub struct Adc<S: Syscalls>(S);

impl<S: Syscalls> Adc<S> {
    pub fn count(x: &mut u32) -> Result<(), ErrorCode> {
        match S::command(DRIVER_NUM, COUNT, 0, 0).get_success_u32() {
            Some(value) => {
                *x = value;
                Ok(())
            }
            None => Err(ErrorCode::Fail),
        }
    }

    pub fn exists() -> Result<(), ErrorCode> {
        let mut check = 0;
        match Self::count(&mut check) {
            Ok(_) if check >= 1 => Ok(()),
            Ok(_) => Err(ErrorCode::Fail),
            Err(_) => Err(ErrorCode::Fail),
        }
    }
}

pub struct AdcListener<F: Fn(i32)>(pub F);
impl<F: Fn(i32)> Upcall<OneId<DRIVER_NUM, 0>> for AdcListener<F> {
    fn upcall(&self, adc_val: u32, _arg1: u32, _arg2: u32) {
        self.0(adc_val as i32)
    }
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x00005;

// Command IDs

const COUNT: u32 = 0;
