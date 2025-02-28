//! `fake` contains fake implementations of Tock kernel components. Fake
//! components emulate the behavior of the real Tock kernel components, but in
//! the unit test environment. They generally have additional testing features,
//! such as error injection functionality.
//!
//! These components are exposed under the `fake` module because otherwise their
//! names would collide with the corresponding drivers (e.g. the fake Console
//! would collide with the Console driver in unit tests). Tests should generally
//! `use libtock_unittest::fake` and refer to the type with the `fake::` prefix
//! (e.g. `fake::Console`).

mod adc;
mod alarm;
mod ambient_light;
mod buttons;
mod buzzer;
mod console;
mod gpio;
mod kernel;
mod leds;
mod low_level_debug;
mod ninedof;
mod proximity;
mod sound_pressure;
mod syscall_driver;
mod syscalls;
mod temperature;

pub use adc::Adc;
pub use alarm::Alarm;
pub use ambient_light::AmbientLight;
pub use buttons::Buttons;
pub use buzzer::Buzzer;
pub use console::Console;
pub use gpio::{Gpio, GpioMode, InterruptEdge, PullMode};
pub use kernel::Kernel;
pub use leds::Leds;
pub use low_level_debug::{LowLevelDebug, Message};
pub use ninedof::{NineDof, NineDofData};
pub use proximity::Proximity;
pub use sound_pressure::SoundPressure;
pub use syscall_driver::SyscallDriver;
pub use syscalls::Syscalls;
pub use temperature::Temperature;

#[cfg(test)]
mod kernel_tests;
