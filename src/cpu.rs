use crate::bus::Bus;

const REGISTER_SIZE: usize = 8;
const STACK_SIZE: usize = 16;

struct CPU {
    /// 4KB of memory
    /// 16 general purpose registers of 8 bit each
    /// 16 levels of stack 
    /// I -> Generally used to store memory addresses
    /// PC -> Program Counter
    /// S
    /// 
    pub registers: [u8; REGISTER_SIZE],             // 16 8 bit general purpose registers
    pub stack: [u16; STACK_SIZE],               // 16 levels of stack for function calls
    pub I : u16,                        // Special register used to store addresses
    pub PC: u16,                        // Program Counter
    pub SP: u8,                         // Stack Pointer
    pub DT: u8,                         // Delay Timer (Automatically decremented at a rate of 60Hz if set)
    pub ST: u8,                         // Sound Timer (Automatically decremented at a rate of 60Hz if set) Buzzer will soudn if ST > 0
    bus: Bus
}

impl CPU {
    pub fn new(bus: Bus) -> CPU {
        CPU {
            registers: [0; REGISTER_SIZE],
            stack: [0; STACK_SIZE],
            I: 0,
            PC: 0,
            SP: 0,
            DT: 0,
            ST: 0,
            bus: bus
        }
    }

    pub fn read_memory(&self, address: &u16) -> u8 {
        self.bus.memory.read(&address)
    }

    pub fn write_to_memory(&mut self, address: &u16, data: &u8) {
        self.bus.memory.write(&address, &data);
    }

}