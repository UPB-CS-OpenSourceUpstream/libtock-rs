#![no_std]

use core::cell::Cell;
use libtock_platform::{
    share::scope, subscribe::OneId, DefaultConfig, ErrorCode, Syscalls, Upcall,
};

pub struct TextScreen<S: Syscalls>(S);

impl<S: Syscalls> TextScreen<S> {
    pub fn exists() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, EXISTS, 0, 0).to_result()
    }

    pub fn turn_display_on() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, DISPLAY, 0, 0).to_result()
    }

    pub fn turn_display_off() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, NO_DISPLAY, 0, 0).to_result()
    }

    pub fn blink_display() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, BLINK, 0, 0).to_result()
    }

    pub fn no_blink_display() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, NO_BLINK, 0, 0).to_result()
    }

    pub fn show_cursor() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, SHOW_CURSOR, 0, 0).to_result()
    }

    pub fn hide_cursor() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, NO_CURSOR, 0, 0).to_result()
    }

    pub fn set_cursor(x: u32, y: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, SET_CURSOR, x, y).to_result()
    }

    pub fn clear_display() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, CLEAR, 0, 0).to_result()
    }

    pub fn home() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, HOME, 0, 0).to_result()
    }

    pub fn get_resolution() -> Result<(u32, u32), ErrorCode> {
        let res_cell: Cell<Option<(u32, u32)>> = Cell::new(None);
        let listener = TextScreenListener(|(width, height)| {
            res_cell.set(Some((width, height)));
        });

        scope(|subscribe| {
            S::subscribe::<_, _, DefaultConfig, DRIVER_NUM, 0>(subscribe, &listener)?;

            Self::get_resolution()?;

            while res_cell.get() == None {
                S::yield_wait();
            }

            match res_cell.get() {
                None => Err(ErrorCode::Fail),
                Some((width, height)) => Ok((width, height)),
            }
        })
    }
}

struct TextScreenListener<F: Fn((u32, u32))>(pub F);
impl<F: Fn((u32, u32))> Upcall<OneId<DRIVER_NUM, 0>> for TextScreenListener<F> {
    fn upcall(&self, _: u32, width: u32, height: u32) {
        self.0((width, height))
    }
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x90003;

// Command IDs

const EXISTS: u32 = 0;
const GET_RESOLUTION: u32 = 1;
const DISPLAY: u32 = 2;
const NO_DISPLAY: u32 = 3;
const BLINK: u32 = 4;
const NO_BLINK: u32 = 5;
const SHOW_CURSOR: u32 = 6;
const NO_CURSOR: u32 = 7;
const WRITE: u32 = 8;
const CLEAR: u32 = 9;
const HOME: u32 = 10;
const SET_CURSOR: u32 = 11;
