#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;

use libtock::adc::Adc;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x200}

fn main() {
    // let mut data = 0;
    match Adc::exists() {
        Ok(()) => writeln!(Console::writer(), "adc driver available").unwrap(),
        Err(_) => {
            writeln!(Console::writer(), "adc driver unavailable").unwrap();
            return;
        }
    }
}
