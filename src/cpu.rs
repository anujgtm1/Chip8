use crate::bus::Bus;
use crate::instructions::Instruction;

const REGISTER_SIZE: usize = 8;
const STACK_SIZE: usize = 16;

struct CPU {
    pub registers: [u8; REGISTER_SIZE], // 16 8 bit general purpose registers
    pub stack: [u16; STACK_SIZE],       // 16 levels of stack for function calls
    pub I : u16,                        // Special register used to store addresses
    pub VF: u16,                        // Flag Register
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
            VF: 0,
            PC: 0,
            SP: 0,
            DT: 0,
            ST: 0,
            bus: bus
        }
    }

    pub fn mock() -> CPU {
        CPU::new(Bus::mock())
    }

    pub fn fetch_opcode(&mut self) -> u16 {
        let instruction = self.bus.memory.read_instruction(&self.PC);
        instruction
    }

    pub fn decode(&self, opcode: u16) -> Instruction {
        Instruction::fetch_opcode(&opcode)
    }

    pub fn inc_pc(&mut self) {
        self.PC += 2;
    }

    pub fn set_pc(&mut self, pc: &u16) {
        self.PC = *pc;
    }

}

#[cfg(test)]
mod test {
    use super::*;

    fn get_cpu() -> CPU {
        let mut cpu = CPU::mock();
        let data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
        cpu.bus.memory.load_data(&0x200, &data);
        cpu
    }

    #[test]
    fn test_fetch_opcode() {
        let mut cpu: CPU = get_cpu();
        cpu.set_pc(&0x200);
        assert_eq!(cpu.fetch_opcode(), 0xDEAD);
        assert_ne!(cpu.fetch_opcode(), 0xBEEF);
        cpu.inc_pc();
        assert_eq!(cpu.fetch_opcode(), 0xBEEF);
    }

    #[test]
    fn test_fetch_decode() {
        let mut cpu: CPU = get_cpu();
        cpu.set_pc(&0x200);
        let opcode: u16 = cpu.fetch_opcode();
        let ins: Instruction = cpu.decode(opcode);
        assert_eq!(ins, Instruction::DisplaySpriteAtLocation{x: 0xE, y: 0xA, n: 0xD});
    }
}