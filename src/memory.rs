use std::convert::TryInto;
use byteorder::{BigEndian, ByteOrder};


static FONT_START: u16 = 0;
static FONT_SIZE: u8 = 5;

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

    pub fn load_fonts(&mut self) {
        let mut i: usize = FONT_START.try_into().unwrap();
        while i < FONT_SET.len(){
            self.memory[i] = FONT_SET[i];
            i += 1;
        }
    }

    pub fn load_data(&mut self, start_address: &u16, data: &Vec<u8>) {
        let mut address: u16 = *start_address;
        for i in 0..data.len() {
            self.write(&address, &data[i]);
            address += 1;
        }
    }

    pub fn get_font_address(&self, font: &u8) -> u16 {
        FONT_START + *font as u16 * FONT_SIZE as u16
    }

    pub fn read_instruction(&self, address: &u16) -> u16 {
        BigEndian::read_u16(&self.memory[*address as usize..])
    }

}

static FONT_SET: [u8; 80] = [
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_read_write() {
        let mut memory = Memory::new();
        memory.write(&0, &0xff);
        assert_eq!(memory.read(&0), 0xff);
    }

    #[test]
    fn test_font_load() {
        let mut memory = Memory::new();
        memory.load_fonts();
        assert_eq!(memory.read(&0), 0xF0);
        assert_eq!(memory.read(&5), 0x20);
        assert_eq!(memory.read(&11), 0x10);
        assert_eq!(memory.read(&17), 0xF0);
    }

    #[test]
    fn test_font_address() {
        let mut memory = Memory::new();
        memory.load_fonts();
        assert_eq!(memory.get_font_address(&0x0), 0);
        assert_eq!(memory.get_font_address(&0x1), 5);
        assert_eq!(memory.get_font_address(&0x2), 10);
        assert_eq!(memory.get_font_address(&0x3), 15);
        assert_eq!(memory.get_font_address(&0x4), 20);
        assert_eq!(memory.get_font_address(&0x5), 25);
        assert_eq!(memory.get_font_address(&0x6), 30);
        assert_eq!(memory.get_font_address(&0x7), 35);
        assert_eq!(memory.get_font_address(&0x8), 40);
        assert_eq!(memory.get_font_address(&0x9), 45);
        assert_eq!(memory.get_font_address(&0xA), 50);
        assert_eq!(memory.get_font_address(&0xB), 55);
        assert_eq!(memory.get_font_address(&0xC), 60);
        assert_eq!(memory.get_font_address(&0xD), 65);
        assert_eq!(memory.get_font_address(&0xE), 70);
        assert_eq!(memory.get_font_address(&0xF), 75);
    }

    #[test]
    fn test_load_data() {
        let mut memory = Memory::new();
        let data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
        memory.load_data(&0x200, &data);
        assert_eq!(memory.read(&0x200), 0xDE);
        assert_eq!(memory.read(&0x201), 0xAD);
        assert_eq!(memory.read(&0x202), 0xBE);
        assert_eq!(memory.read(&0x203), 0xEF);
    }

    #[test]
    fn test_read_instruction() {
        let mut memory = Memory::new();
        let data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
        memory.load_data(&0x200, &data);
        assert_eq!(memory.read_instruction(&0x200), 0xDEAD);
        assert_eq!(memory.read_instruction(&0x202), 0xBEEF);
    }
}