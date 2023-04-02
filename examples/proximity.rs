
#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;

use libtock::alarm::{Alarm, Milliseconds};
use libtock::runtime::{set_main, stack_size};
use libtock::proximity::Proximity;

set_main! {main}
stack_size! {0x200}

fn main() {
    match Proximity::exists() {
        Ok(()) => writeln!(Console::writer(), "proxi driver available").unwrap(),
        Err(_) => {
            writeln!(Console::writer(), "proxi driver unavailable").unwrap();
            return;
        }
    }

    loop {
        match Proximity::read_proximity() {
            Ok(temp_val) => writeln!(
                Console::writer(),
                "Temperature: {}{}.{}*C\n",
                if temp_val > 0 { "" } else { "-" },
                (temp_val) / 100,
                (temp_val) % 100
            )
            .unwrap(),
            Err(_) => writeln!(Console::writer(), "error while reading proxi",).unwrap(),
        
        }

        Alarm::sleep_for(Milliseconds(2000)).unwrap();
    }
}