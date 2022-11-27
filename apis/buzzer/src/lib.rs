#![no_std]

use libtock_platform::{ErrorCode, Syscalls};

pub struct Buzzer<S: Syscalls>(S);

impl<S: Syscalls> Buzzer<S> {

    pub fn tone(d1: u32, d2: f64) {
        S::command(DRIVER_NUM, d1, d2, 0).to_result()
    }

    pub fn buzzer_exists() -> bool {
        S::command(DRIVER_NUM, 0, 0, 0).to_result()
    }
}

#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x90000;

// Command IDs aka NOTES
const NOTE_B0 : u32 31;
const NOTE_C1 : u32 33;
const NOTE_CS1: u32 35;
const NOTE_D1 : u32 37;
const NOTE_DS1: u32 39;
const NOTE_E1 : u32 41;
const NOTE_F1 : u32 44;
const NOTE_FS1: u32 46;
const NOTE_G1 : u32 49;
const NOTE_GS1: u32 52;
const NOTE_A1 : u32 55;
const NOTE_AS1: u32 58;
const NOTE_B1 : u32 62;
const NOTE_C2 : u32 65;
const NOTE_CS2: u32 69;
const NOTE_D2 : u32 73;
const NOTE_DS2: u32 78;
const NOTE_E2 : u32 82;
const NOTE_F2 : u32 87;
const NOTE_FS2: u32 93;
const NOTE_G2 : u32 98;
const NOTE_GS2: u32 104;
const NOTE_A2 : u32 110;
const NOTE_AS2: u32 117;
const NOTE_B2 : u32 123;
const NOTE_C3 : u32 131;
const NOTE_CS3: u32 139;
const NOTE_D3 : u32 147;
const NOTE_DS3: u32 156;
const NOTE_E3 : u32 165;
const NOTE_F3 : u32 175;
const NOTE_FS3: u32 185;
const NOTE_G3 : u32 196;
const NOTE_GS3: u32 208;
const NOTE_A3 : u32 220;
const NOTE_AS3: u32 233;
const NOTE_B3 : u32 247;
const NOTE_C4 : u32 262;
const NOTE_CS4: u32 277;
const NOTE_D4 : u32 294;
const NOTE_DS4: u32 311;
const NOTE_E4 : u32 330;
const NOTE_F4 : u32 349;
const NOTE_FS4: u32 370;
const NOTE_G4 : u32 392;
const NOTE_GS4: u32 415;
const NOTE_A4 : u32 440;
const NOTE_AS4: u32 466;
const NOTE_B4 : u32 494;
const NOTE_C5 : u32 523;
const NOTE_CS5: u32 554;
const NOTE_D5 : u32 587;
const NOTE_DS5: u32 622;
const NOTE_E5 : u32 659;
const NOTE_F5 : u32 698;
const NOTE_FS5: u32 740;
const NOTE_G5 : u32 784;
const NOTE_GS5: u32 831;
const NOTE_A5 : u32 880;
const NOTE_AS5: u32 932;
const NOTE_B5 : u32 988;
const NOTE_C6 : u32 1047;
const NOTE_CS6: u32 1109;
const NOTE_D6 : u32 1175;
const NOTE_DS6: u32 1245;
const NOTE_E6 : u32 1319;
const NOTE_F6 : u32 1397;
const NOTE_FS6: u32 1480;
const NOTE_G6 : u32 1568;
const NOTE_GS6: u32 1661;
const NOTE_A6 : u32 1760;
const NOTE_AS6: u32 1865;
const NOTE_B6 : u32 1976;
const NOTE_C7 : u32 2093;
const NOTE_CS7: u32 2217;
const NOTE_D7 : u32 2349;
const NOTE_DS7: u32 2489;
const NOTE_E7 : u32 2637;
const NOTE_F7 : u32 2794;
const NOTE_FS7: u32 2960;
const NOTE_G7 : u32 3136;
const NOTE_GS7: u32 3322;
const NOTE_A7 : u32 3520;
const NOTE_AS7: u32 3729;
const NOTE_B7 : u32 3951;
const NOTE_C8 : u32 4186;
const NOTE_CS8: u32 4435;
const NOTE_D8 : u32 4699;
const NOTE_DS8: u32 4978;

