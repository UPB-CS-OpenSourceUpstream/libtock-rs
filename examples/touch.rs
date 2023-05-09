//! A simple libtock-rs example. Checks for touch driver
//! and waits for single touch events, printing their location

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;

use libtock::runtime::{set_main, stack_size};
use libtock::touch::Touch;

set_main! {main}
stack_size! {0x200}

fn main() {
    if Touch::exists().is_err() {
        writeln!(Console::writer(), "touch driver unavailable").unwrap();
        return;
    }

    loop {
        match Touch::wait_for_single_touch() {
            Err(_) => writeln!(Console::writer(), "Error in getting touch event").unwrap(),
            Ok(event) => writeln!(
                Console::writer(),
                "Touch event at ({}, {})",
                event.x,
                event.y
            )
            .unwrap(),
        }
    }
}
