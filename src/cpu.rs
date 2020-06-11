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
    pub SP: usize,                         // Stack Pointer
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

    pub fn execute_opcode(&mut self, ins: Instruction) {
        let mut inc_pc = true;
        match ins {
            Instruction::NOP => {},
            Instruction::InvalidInstruction => {},
            Instruction::Jump{address} => {
                self.PC = address;
                inc_pc = false;
            },
            Instruction::Call{address} => {
                inc_pc = false;
                self.stack[self.SP] = self.PC;
                self.SP += 1;
                self.PC = address;
            },
            Instruction::Return => {
                inc_pc = false;
                self.SP -= 1;
                self.PC = self.stack[self.SP];
            },
            Instruction::SkipIfEqual{register, value} => {
                if self.registers[register as usize] == value {
                    self.inc_pc();
                }
            },
            Instruction::SkipIfNotEqual{register, value} => {
                if self.registers[register as usize] != value {
                    self.inc_pc();
                }
            },
            Instruction::SkipIfRegistersEqual{register_1, register_2} => {
                if self.registers[register_1 as usize] == self.registers[register_2 as usize] {
                    self.inc_pc();
                }
            },
            Instruction::LoadRegister{register, value} => {
                self.registers[register as usize] = value;
            },
            Instruction::AddToRegister{register, value} => {
                self.registers[register as usize] += value;
            }
            
            _ => {}
        };
        if inc_pc {
            self.inc_pc();
        }
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

    fn test_pc_inc(init: u16, cpu: CPU) {
        assert_eq!(cpu.PC, init + 2);
    }

    #[test]
    fn test_nop() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::NOP);
        test_pc_inc(0, cpu);
    }

    #[test]
    fn test_skip_invalid_instruction() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::InvalidInstruction);
        test_pc_inc(0, cpu);
    }

    #[test]
    fn test_jump() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::Jump{address: 0xFFF});
        assert_eq!(cpu.PC, 0xFFF);
    }

    #[test]
    fn test_call() {
        let mut cpu = get_cpu();
        cpu.set_pc(&0x0E0);
        cpu.execute_opcode(Instruction::Call{address: 0x0FF});
        assert_eq!(cpu.SP, 1);
        assert_eq!(cpu.stack[0], 0x00E0);
        assert_eq!(cpu.PC, 0x0FF);
    }

    #[test]
    fn test_return() {
        let mut cpu = get_cpu();
        cpu.set_pc(&0x0E0);
        cpu.execute_opcode(Instruction::Call{address: 0x0FF});
        cpu.execute_opcode(Instruction::Call{address: 0x0DD});
        cpu.execute_opcode(Instruction::Return);
        assert_eq!(cpu.SP, 1);
        assert_eq!(cpu.PC, 0x0FF);
        assert_eq!(cpu.stack[0], 0x0E0);
    }

    #[test]
    fn test_skip_if_equal() {
        let mut cpu = get_cpu();
        cpu.registers[5] = 0x05;
        cpu.execute_opcode(Instruction::SkipIfEqual{register: 5, value: 0x2});
        assert_eq!(cpu.PC, 0x02);
        cpu.execute_opcode(Instruction::SkipIfEqual{register: 5, value: 0x05});
        assert_eq!(cpu.PC, 0x06);
    }

    #[test]
    fn test_skip_if_not_equal() {
        let mut cpu = get_cpu();
        cpu.registers[5] = 0x05;
        cpu.execute_opcode(Instruction::SkipIfNotEqual{register: 5, value: 0x2});
        assert_eq!(cpu.PC, 0x04);
        cpu.execute_opcode(Instruction::SkipIfNotEqual{register: 5, value: 0x05});
        assert_eq!(cpu.PC, 0x06);
    }

    #[test]
    fn test_skip_if_registers_equal() {
        let mut cpu = get_cpu();
        cpu.registers[5] = 0x05;
        cpu.registers[4] = 0x04;
        cpu.registers[3] = 0x05;
        cpu.execute_opcode(Instruction::SkipIfRegistersEqual{register_1: 5, register_2: 4});
        assert_eq!(cpu.PC, 0x02);
        cpu.execute_opcode(Instruction::SkipIfRegistersEqual{register_1: 5, register_2: 3});
        assert_eq!(cpu.PC, 0x06);
    }

    #[test]
    fn test_load_register() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xBB});
        assert_eq!(cpu.registers[5], 0xBB);
    }

    #[test]
    fn test_add_to_register() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xB});
        cpu.execute_opcode(Instruction::AddToRegister{register: 5, value: 0xB0});
        assert_eq!(cpu.registers[5], 0xBB);
    }

    #[test]
    fn test_set_reg_to_reg() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xBB});
        cpu.execute_opcode(Instruction::SetRegisterToRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0xBB);
    }

    #[test]
    fn test_or_reg_reg() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xB1});
        cpu.execute_opcode(Instruction::LoadRegister{register: 4, value: 0x1B});
        cpu.execute_opcode(Instruction::OrRegisterToRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0xBB);
    }

    #[test]
    fn test_and_reg_reg() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xB1});
        cpu.execute_opcode(Instruction::LoadRegister{register: 4, value: 0x1B});
        cpu.execute_opcode(Instruction::AndRegisterToRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0x11);
    }

    #[test]
    fn test_xor_reg_reg() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xB1});
        cpu.execute_opcode(Instruction::LoadRegister{register: 4, value: 0x1B});
        cpu.execute_opcode(Instruction::XorRegisterToRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0xAA);
    }

    
    #[test]
    fn test_add_reg_reg() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xB0});
        cpu.execute_opcode(Instruction::LoadRegister{register: 4, value: 0x0B});
        cpu.execute_opcode(Instruction::AddRegisterToRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0xBB);
        assert_eq!(cpu.VF, 0);
        cpu.execute_opcode(Instruction::AddRegisterToRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.VF, 1);
    }

    #[test]
    fn test_sub_reg_reg() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xB0});
        cpu.execute_opcode(Instruction::LoadRegister{register: 4, value: 0xBB});
        cpu.execute_opcode(Instruction::SubtractRegisterFromRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0x0B);
        assert_eq!(cpu.VF, 0);
        cpu.execute_opcode(Instruction::SubtractRegisterFromRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.VF, 1)
    }

    #[test]
    fn test_sub_reg_reg_inv() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0xBB});
        cpu.execute_opcode(Instruction::LoadRegister{register: 4, value: 0xB0});
        cpu.execute_opcode(Instruction::SubtractIntoDifferentRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0x0B);
        assert_eq!(cpu.VF, 0);
        cpu.execute_opcode(Instruction::SubtractIntoDifferentRegister{destination_register: 4, source_register: 5});
        assert_eq!(cpu.VF, 1)
    }

    #[test]
    fn test_shr() {
        let mut cpu = get_cpu();
        cpu.execute_opcode(Instruction::LoadRegister{register: 5, value: 0x22});
        cpu.execute_opcode(Instruction::ShiftRight{destination_register: 4, source_register: 5});
        assert_eq!(cpu.registers[4], 0x11);
        assert_eq!(cpu.VF, 0);
        cpu.execute_opcode(Instruction::ShiftRight{destination_register: 4, source_register: 4});
        assert_eq!(cpu.VF, 1);
        assert_eq!(cpu.registers[4], 0x08);
    }

}