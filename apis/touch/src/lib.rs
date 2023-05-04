#![no_std]

use core::cell::Cell;
use libtock_platform::{share, DefaultConfig, ErrorCode, Subscribe, Syscalls};
pub struct Touch<S: Syscalls>(S);

pub enum TouchStatus {
    Unstarted,
    Pressed,
    Released,
    Moved,
}

impl TouchStatus {
    fn from_u32(value: u32) -> TouchStatus {
        match value {
            0 => TouchStatus::Released,
            1 => TouchStatus::Pressed,
            2 => TouchStatus::Moved,
            _ => TouchStatus::Unstarted,
        }
    }
}

pub struct TouchEvent {
    /// touch event type
    pub status: TouchStatus,
    /// touch (x, y) position
    pub x: u16,
    pub y: u16,
    /// A scaled value for the size of the touch, if the touchscreen offers that information,
    /// or None otherwise.
    /// A larger value corresponds to a "fatter" touch.
    pub area: Option<u16>,
    /// A scaled value for the pressure of the touch, if the touchscreen offers that information,
    /// or None otherwise.
    /// A larger value corresponds to a "firmer" press.
    pub pressure: Option<u16>,
}

pub enum GestureEvent {
    SwipeUp,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
    ZoomIn,
    ZoomOut,
}

impl<S: Syscalls> Touch<S> {
    pub fn exists() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, command::DRIVER_CHECK, 0, 0).to_result()
    }

    pub fn enable_single_touch() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, command::ENABLE_SINGLE, 0, 0).to_result()
    }

    pub fn disable_single_touch() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, command::DISABLE_SINGLE, 0, 0).to_result()
    }

    pub fn register_single_touch_listener<'share>(
        listener: &'share Cell<Option<(u32, u32, u32)>>,
        subscribe: share::Handle<Subscribe<'share, S, DRIVER_NUM, { subscribe::SINGLE }>>,
    ) -> Result<(), ErrorCode> {
        S::subscribe::<_, _, DefaultConfig, DRIVER_NUM, { subscribe::SINGLE }>(subscribe, listener)
    }

    /// Waits for a single touch event, returning details about it in a TouchEvent structure:
    pub fn wait_for_single_touch() -> Result<TouchEvent, ErrorCode> {
        Self::enable_single_touch()?;
        let listener: Cell<Option<(u32, u32, u32)>> = Cell::new(None);
        share::scope(|subscribe| {
            if let Ok(()) = Self::register_single_touch_listener(&listener, subscribe) {
                while listener.get().is_none() {
                    S::yield_wait();
                }
            }
        });
        match listener.get() {
            None => Err(ErrorCode::Fail),
            Some(tuple) => Ok(TouchEvent {
                status: TouchStatus::from_u32(tuple.0),
                x: (tuple.1 >> 16) as u16,
                y: tuple.1 as u16,
                area: match tuple.2 as u16 {
                    0 => None,
                    val => Some(val),
                },
                pressure: match (tuple.2 >> 16) as u16 {
                    0 => None,
                    val => Some(val),
                },
            }),
        }
    }
    
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 589826;

// Command IDs

mod command {
    pub const DRIVER_CHECK: u32 = 0;
    pub const ENABLE_SINGLE: u32 = 1;
    pub const DISABLE_SINGLE: u32 = 2;
    pub const ACK_MULTI: u32 = 10;
    pub const ENABLE_MULTI: u32 = 11;
    pub const DISABLE_MULTI: u32 = 12;
    pub const TOUCHES_NUM: u32 = 100;
}

mod subscribe {
    pub const SINGLE: u32 = 0;
    pub const GESTURES: u32 = 1;
    pub const MULTI: u32 = 2;
}

mod allow_rw {
    pub const MULTI: u32 = 0;
}
