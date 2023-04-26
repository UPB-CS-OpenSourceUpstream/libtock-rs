//! A simple libtock-rs example. Checks for humidity driver
//! and samples the sensor every 2 seconds.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;

use libtock::alarm::{Alarm, Milliseconds};
use libtock::humidity::Humidity;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x200}

fn main() {
    if Humidity::exists().is_err() {
        writeln!(Console::writer(), "humidity driver unavailable").unwrap();
        return;
    }

    loop {
        match Humidity::read_sync() {
            Ok(hum_val) => writeln!(Console::writer(), "Humidity: {}%\n", hum_val).unwrap(),
            Err(_) => writeln!(Console::writer(), "error while reading humidity",).unwrap(),
        }

        Alarm::sleep_for(Milliseconds(2000)).unwrap();
    }
}
