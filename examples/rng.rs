//! A simple libtock-rs example. Checks for random number generator driver
//! asks for some random numbers and prints them.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::rng::Rng;
use libtock::runtime::{set_main, stack_size};
set_main! {main}
stack_size! {0x200}

fn main() {
    if Rng::exists().is_err() {
        writeln!(
            Console::writer(),
            "Random number generator driver unavailable"
        )
        .unwrap();
        return;
    }

    writeln!(
        Console::writer(),
        "Random number generator driver available"
    )
    .unwrap();

    let mut buf: [u8; 10] = [0; 10];

    match Rng::get_random_sync(&mut buf[..], 20) {
        Ok(bytes_received) => {
            writeln!(
                Console::writer(),
                "Received {} random bytes. Buf:",
                bytes_received,
            )
            .unwrap();
            for byte in buf.iter() {
                writeln!(Console::writer(), "{} ", byte).unwrap();
            }
        }
        Err(_) => {
            writeln!(Console::writer(), "FAIL").unwrap();
        }
    }
}
