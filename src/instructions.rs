type Address = u16;
type HalfWord = u8;


#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    NOP,
    InvalidInstruction,
    ClearDisplay,
    Return,
    Jump{address: u16},
    Call{address: u16},
    SkipIfEqual{register: u8, value: u8},
    SkipIfNotEqual{register: u8, value: u8},
    SkipIfRegistersEqual{register_1: u8, register_2: u8},
    LoadRegister{register: u8, value: u8},
    AddToRegister{register: u8, value: u8},
    SetRegisterToRegister{destination_register: u8, source_register: u8},
    OrRegisterToRegister{destination_register: u8, source_register: u8},
    AndRegisterToRegister{destination_register: u8, source_register: u8},
    XorRegisterToRegister{destination_register: u8, source_register: u8},
    AddRegisterToRegister{destination_register: u8, source_register: u8},
    SubtractRegisterFromRegister{destination_register: u8, source_register: u8},
    ShiftRight{destination_register: u8, source_register: u8},
    SubtractIntoDifferentRegister{source_register: u8, destination_register: u8},
    ShiftLeft{destination_register: u8, source_register: u8},
    SkipIfRegistersNotEqual{register_1: u8, register_2: u8},
    SetAddressRegister{value: u16},
    JumpToLocationAndOffset0{address: u16},
    GenerateRandomData{register: u8, value: u8},
    DisplaySpriteAtLocation{x: u8, y: u8, n: u8},
    SkipIfPressedKeyEqualToRegister{register: u8},
    DontSkipIfPressedKeyEqualToRegister{register: u8},
    SetRegisterToDelayTimer{register: u8},
    WaitForKeyPressAndStoreValue{register: u8},
    SetDelayTimerToRegister{register: u8},
    SetSoundTimerToRegister{register: u8},
    AddRegisterToRegisterI{register: u8},
    SetIToFontAddress{digit: u8},
    StoreBCDValueOfRegisterToI{register: u8},
    StoreNRegistersToMemory{n: u8},
    ReadNRegistersFromMemory{n: u8}
}

fn get_first_nibble(value: &u16) -> u8 {
    (value >> 12) as u8
}

fn get_second_nibble(value: &u16) -> u8 {
    (value << 4 >> 12) as u8
}

fn get_third_nibble(value: &u16) -> u8 {
    (value << 8 >> 12) as u8
}

fn get_last_nibble(value: &u16) -> u8 {
    (value << 12 >> 12) as u8
}

fn get_last_byte(value: &u16) -> u8 {
    (value << 8 >> 8) as u8
}

fn get_first_byte(value: &u16) -> u8 {
    (value >> 8) as u8
}

fn get_last_3_nibbles(value: &u16) -> u16 {
    (value << 4 >> 4)
}


impl Instruction {
    pub fn fetch_opcode(word: &u16) -> Instruction {
        match get_first_nibble(&word) {
            0 => match get_last_byte(&word) {
                0xE0 => Instruction::ClearDisplay,
                0xEE => Instruction::Return,
                _ => Instruction::NOP
            },
            1 => Instruction::Jump{address: get_last_3_nibbles(&word)},
            2 => Instruction::Call{address: get_last_3_nibbles(&word)},
            3 => Instruction::SkipIfEqual{register: get_second_nibble(&word), value: get_last_byte(&word)},
            4 => Instruction::SkipIfNotEqual{register: get_second_nibble(&word), value: get_last_byte(&word)},
            5 => match get_last_nibble(&word) {
                0 => Instruction::SkipIfRegistersEqual{register_1: get_second_nibble(&word), register_2: get_third_nibble(&word)},
                _ => Instruction::InvalidInstruction
            }
            6 => Instruction::LoadRegister{register: get_second_nibble(&word), value: get_last_byte(&word)},
            7 => Instruction::AddToRegister{register: get_second_nibble(&word), value: get_last_byte(&word)},
            8 => {
                let register_1: u8 = get_second_nibble(&word);
                let register_2: u8 = get_third_nibble(&word);
                match get_last_nibble(&word) {
                    0 => Instruction::SetRegisterToRegister{destination_register: register_1, source_register: register_2},
                    1 => Instruction::OrRegisterToRegister{destination_register: register_1, source_register: register_2},
                    2 => Instruction::AndRegisterToRegister{destination_register: register_1, source_register: register_2},
                    3 => Instruction::XorRegisterToRegister{destination_register: register_1, source_register: register_2},
                    4 => Instruction::AddRegisterToRegister{destination_register: register_1, source_register: register_2},
                    5 => Instruction::SubtractRegisterFromRegister{destination_register: register_1, source_register: register_2},
                    6 => Instruction::ShiftRight{destination_register: register_1, source_register: register_2},
                    7 => Instruction::SubtractIntoDifferentRegister{source_register: register_1, destination_register: register_2},
                    0xE => Instruction::ShiftLeft{destination_register: register_1, source_register: register_2},
                    _ => Instruction::InvalidInstruction
                }
            }
            9 => Instruction::SkipIfRegistersNotEqual{register_1: get_second_nibble(&word), register_2: get_third_nibble(&word)},
            0xA => Instruction::SetAddressRegister{value: get_last_3_nibbles(&word)},
            0xB => Instruction::JumpToLocationAndOffset0{address: get_last_3_nibbles(&word)},
            0xC => Instruction::GenerateRandomData{register: get_second_nibble(&word), value: get_last_byte(&word)},
            0xD => Instruction::DisplaySpriteAtLocation{x: get_second_nibble(&word), y: get_third_nibble(&word), n: get_last_nibble(&word)},
            0xE => {
                match get_last_byte(&word) {
                    0x9E => Instruction::SkipIfPressedKeyEqualToRegister{register: get_second_nibble(&word)},
                    0xA1 => Instruction::DontSkipIfPressedKeyEqualToRegister{register: get_second_nibble(&word)},
                    _ => Instruction::InvalidInstruction
                }
            },
            0xF => {
                match get_last_byte(&word) {
                    0x07 => Instruction::SetRegisterToDelayTimer{register: get_second_nibble(&word)},
                    0x0A => Instruction::WaitForKeyPressAndStoreValue{register: get_second_nibble(&word)},
                    0x15 => Instruction::SetDelayTimerToRegister{register: get_second_nibble(&word)},
                    0x18 => Instruction::SetSoundTimerToRegister{register: get_second_nibble(&word)},
                    0x1E => Instruction::AddRegisterToRegisterI{register: get_second_nibble(&word)},
                    0x29 => Instruction::SetIToFontAddress{digit: get_second_nibble(&word)},
                    0x33 => Instruction::StoreBCDValueOfRegisterToI{register:get_second_nibble(&word)},
                    0x55 => Instruction::StoreNRegistersToMemory{n: get_second_nibble(&word)},
                    0x65 => Instruction::ReadNRegistersFromMemory{n: get_second_nibble(&word)},
                    _ => Instruction::InvalidInstruction
                }
            },
            _ => Instruction::InvalidInstruction
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_nibble() {
        let data = 0xDEAD;
        assert_eq!(get_first_nibble(&data), 0xD);
        assert_eq!(get_second_nibble(&data), 0xE);
        assert_eq!(get_third_nibble(&data), 0xA);
        assert_eq!(get_last_nibble(&data), 0xD);
        assert_eq!(get_last_byte(&data), 0xAD);
        assert_eq!(get_last_3_nibbles(&data), 0xEAD);
    }

    #[test]
    fn test_fetch_clear_display() {
        assert_eq!(Instruction::fetch_opcode(&0x00E0), Instruction::ClearDisplay);
    }

    #[test]
    fn test_return() {
        assert_eq!(Instruction::fetch_opcode(&0x00EE), Instruction::Return);
    }

    #[test]
    fn test_nop() {
        assert_eq!(Instruction::fetch_opcode(&0x0000), Instruction::NOP);
    }

    #[test]
    fn test_invalid_instruction() {
        assert_eq!(Instruction::fetch_opcode(&0xF000), Instruction::InvalidInstruction);
        assert_eq!(Instruction::fetch_opcode(&0xE000), Instruction::InvalidInstruction);
        assert_eq!(Instruction::fetch_opcode(&0x8009), Instruction::InvalidInstruction);
        assert_eq!(Instruction::fetch_opcode(&0x5001), Instruction::InvalidInstruction);
    }

    #[test]
    fn test_jump_to_location() {
        assert_eq!(Instruction::fetch_opcode(&0x1FEF), Instruction::Jump{address: 0xFEF});
    }

    #[test]
    fn test_call() {
        assert_eq!(Instruction::fetch_opcode(&0x2FEF), Instruction::Call{address: 0xFEF});
    }

    #[test]
    fn test_skip_if_equal() {
        assert_eq!(Instruction::fetch_opcode(&0x3355), Instruction::SkipIfEqual{register: 0x3, value: 0x55});
    }

    #[test]
    fn test_skip_if_not_equal() {
        assert_eq!(Instruction::fetch_opcode(&0x4355), Instruction::SkipIfNotEqual{register: 0x3, value: 0x55});
    }

    #[test]
    fn test_skip_if_register_equal() {
        assert_eq!(Instruction::fetch_opcode(&0x5350), Instruction::SkipIfRegistersEqual{register_1: 0x3, register_2: 0x5});
    }

    #[test]
    fn test_load_register() {
        assert_eq!(Instruction::fetch_opcode(&0x6355), Instruction::LoadRegister{register: 0x3, value: 0x55});
    }

    #[test]
    fn test_add_register() {
        assert_eq!(Instruction::fetch_opcode(&0x7344), Instruction::AddToRegister{register: 0x3, value: 0x44});
    }

    #[test]
    fn test_load_register_to_register() {
        assert_eq!(Instruction::fetch_opcode(&0x8350), Instruction::SetRegisterToRegister{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_or_register() {
        assert_eq!(Instruction::fetch_opcode(&0x8351), Instruction::OrRegisterToRegister{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_and_register() {
        assert_eq!(Instruction::fetch_opcode(&0x8352), Instruction::AndRegisterToRegister{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_xor_register() {
        assert_eq!(Instruction::fetch_opcode(&0x8353), Instruction::XorRegisterToRegister{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_add_register_register() {
        assert_eq!(Instruction::fetch_opcode(&0x8354), Instruction::AddRegisterToRegister{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_sub_register_register() {
        assert_eq!(Instruction::fetch_opcode(&0x8355), Instruction::SubtractRegisterFromRegister{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_shr() {
        assert_eq!(Instruction::fetch_opcode(&0x8356), Instruction::ShiftRight{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_sub_register_register_inverse() {
        assert_eq!(Instruction::fetch_opcode(&0x8357), Instruction::SubtractIntoDifferentRegister{destination_register: 0x5, source_register: 0x3});
    }

    #[test]
    fn test_shl() {
        assert_eq!(Instruction::fetch_opcode(&0x835E), Instruction::ShiftLeft{destination_register: 0x3, source_register: 0x5});
    }

    #[test]
    fn test_skip_if_register_not_equal() {
        assert_eq!(Instruction::fetch_opcode(&0x9350), Instruction::SkipIfRegistersNotEqual{register_1: 0x3, register_2: 0x5});
    }

    #[test]
    fn test_set_address_register() {
        assert_eq!(Instruction::fetch_opcode(&0xA351), Instruction::SetAddressRegister{value: 0x351});
    }

    #[test]
    fn test_jump_to_location_offset() {
        assert_eq!(Instruction::fetch_opcode(&0xB351), Instruction::JumpToLocationAndOffset0{address: 0x351});
    }

    #[test]
    fn test_random_byte() {
        assert_eq!(Instruction::fetch_opcode(&0xC351), Instruction::GenerateRandomData{register: 0x3, value: 0x51});
    }

    #[test]
    fn test_draw_sprite() {
        assert_eq!(Instruction::fetch_opcode(&0xD351), Instruction::DisplaySpriteAtLocation{x: 3, y: 5, n: 1});
    }

    #[test]
    fn test_skip_if_key_value_match() {
        assert_eq!(Instruction::fetch_opcode(&0xE39E), Instruction::SkipIfPressedKeyEqualToRegister{register: 0x3});
    }

    #[test]
    fn test_skip_if_not_key_value_match() {
        assert_eq!(Instruction::fetch_opcode(&0xE3A1), Instruction::DontSkipIfPressedKeyEqualToRegister{register: 0x3});
    }
    
    #[test]
    fn test_load_dt_to_register() {
        assert_eq!(Instruction::fetch_opcode(&0xF307), Instruction::SetRegisterToDelayTimer{register: 0x3});
    }

    #[test]
    fn test_wait_for_key_press() {
        assert_eq!(Instruction::fetch_opcode(&0xF30A), Instruction::WaitForKeyPressAndStoreValue{register: 0x3});
    }

    #[test]
    fn test_set_dt_to_register() {
        assert_eq!(Instruction::fetch_opcode(&0xF315), Instruction::SetDelayTimerToRegister{register: 0x3});
    }

    #[test]
    fn test_set_st_to_register() {
        assert_eq!(Instruction::fetch_opcode(&0xF318), Instruction::SetSoundTimerToRegister{register: 0x3});
    }

    #[test]
    fn test_add_register_to_i() {
        assert_eq!(Instruction::fetch_opcode(&0xF31E), Instruction::AddRegisterToRegisterI{register: 0x3});
    }

    #[test]
    fn test_get_font_value() {
        assert_eq!(Instruction::fetch_opcode(&0xF329), Instruction::SetIToFontAddress{digit: 0x3});
    }

    #[test]
    fn test_store_bcd_to_i() {
        assert_eq!(Instruction::fetch_opcode(&0xF333), Instruction::StoreBCDValueOfRegisterToI{register: 0x3});
    }

    #[test]
    fn test_store_n_registers() {
        assert_eq!(Instruction::fetch_opcode(&0xF355), Instruction::StoreNRegistersToMemory{n: 0x3});
    }

    #[test]
    fn test_load_n_registers() {
        assert_eq!(Instruction::fetch_opcode(&0xF365), Instruction::ReadNRegistersFromMemory{n: 0x3});
    }
}