
#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;

use libtock::alarm::{Alarm, Milliseconds};
use libtock::runtime::{set_main, stack_size};
use libtock::sound_pressure::SoundPressure;

set_main! {main}
stack_size! {0x200}

fn main() {
    match SoundPressure::exists() {
        Ok(()) => writeln!(Console::writer(), "proxi driver available").unwrap(),
        Err(_) => {
            writeln!(Console::writer(), "proxi driver unavailable").unwrap();
            return;
        }
    }

    loop {
        match SoundPressure::read_sound_pressure_sync() {
            Ok(temp_val) => writeln!(
                Console::writer(),
                "Pressure: {}\n",
                temp_val
            )
            .unwrap(),
            Err(a) => writeln!(Console::writer(), "error while reading proxi {:?}", a).unwrap(),
        
        }

        Alarm::sleep_for(Milliseconds(2000)).unwrap();
    }
}