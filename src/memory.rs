use std::convert::TryInto;

const FONT_START: u16 = 0;
const FONT_SIZE: u8 = 5;

pub struct Memory {
    pub memory: [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            memory: [0; 4096]
        }
    }
    pub fn read(&self, address: &u16) -> u8 {
        self.memory[*address as usize]
    }

    pub fn write(&mut self, address: &u16, data: &u8) {
        self.memory[*address as usize] = *data;
    }

    pub fn put_fonts(&mut self) {
        let mut i: usize = FONT_START.try_into().unwrap();
        while i < fontset.len(){
            self.memory[i] = fontset[i];
            i += 1;
        }
    }

    pub fn get_font_address(&self, font: u8) -> u16 {
        FONT_START + font as u16 * FONT_SIZE as u16
    }

}

static fontset: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,  // 0
    0x20, 0x60, 0x20, 0x20, 0x70,  // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
    0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
    0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
    0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
    0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
    0xF0, 0x80, 0xF0, 0x80, 0x80   // F
];


// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_write_read_works() {
//         let mut memory = Memory::new();
//         memory.write(address: &u16, data: &u8)
//     }
// }