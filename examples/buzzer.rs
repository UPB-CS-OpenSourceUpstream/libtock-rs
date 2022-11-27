#![no_main]
#![no_std]

use kernel::debug;
use libtock::buzzer::Buzzer;

const TEMPO: u32 = 114;
const MELODY: [i32; 126] = [
    NOTE_E4, 4, NOTE_E4, 4, NOTE_F4, 4, NOTE_G4, 4, NOTE_G4, 4, NOTE_F4, 4, NOTE_E4, 4, NOTE_D4, 4,
    NOTE_C4, 4, NOTE_C4, 4, NOTE_D4, 4, NOTE_E4, 4, NOTE_E4, -4, NOTE_D4, 8, NOTE_D4, 2, NOTE_E4,
    4, NOTE_E4, 4, NOTE_F4, 4, NOTE_G4, 4, NOTE_G4, 4, NOTE_F4, 4, NOTE_E4, 4, NOTE_D4, 4, NOTE_C4,
    4, NOTE_C4, 4, NOTE_D4, 4, NOTE_E4, 4, NOTE_D4, -4, NOTE_C4, 8, NOTE_C4, 2, NOTE_D4, 4,
    NOTE_D4, 4, NOTE_E4, 4, NOTE_C4, 4, NOTE_D4, 4, NOTE_E4, 8, NOTE_F4, 8, NOTE_E4, 4, NOTE_C4, 4,
    NOTE_D4, 4, NOTE_E4, 8, NOTE_F4, 8, NOTE_E4, 4, NOTE_D4, 4, NOTE_C4, 4, NOTE_D4, 4, NOTE_G3, 2,
    NOTE_E4, 4, NOTE_E4, 4, NOTE_F4, 4, NOTE_G4, 4, NOTE_G4, 4, NOTE_F4, 4, NOTE_E4, 4, NOTE_D4, 4,
    NOTE_C4, 4, NOTE_C4, 4, NOTE_D4, 4, NOTE_E4, 4, NOTE_D4, -4, NOTE_C4, 8, NOTE_C4, 2,
];

fn main() {
    let mut cont = 0;

    if buzzer_exists() {
        debug!("There is no buzzer!\n");
        return -1;
    }
    debug!("Ode of Joy\n");
    let notes: i32 = (melody.len()) / 2;
    let wholenote = (60000 * 4) / TEMPO;

    for i in melody {
        if cont % 2 == 0 {
            let divider = &melody[cont + 1];
            let mut note_duration: f64 = 0;

            if divider > 0 {
                note_duration = ((&wholenote) / divider.abs()) as f64;
            } else if divider < 0 {
                note_duration = ((&wholenote) / divider.abs()) as f64;
                note_duration = note_duration * 1.5;
            }

            tone(i * 3, note_duration * 0.9);
        }
        cont = cont + 1;
    }
}
