
type Address = u16;
type HalfWord = u8;


#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearDisplay,
    Return,
    Jump(address: u16),
    Call(address: u16),
    SkipIfEqual(register: u8, value: u16),
    SkipIfNotEqual(register: u8, value: u16),
    SkipIfRegistersEqual(register_1: u8, register_2: u8),
    LoadRegister(register: u8, value: u16),
    AddToRegister(register: u8, value: u16),
    SetRegisterToRegister(destination_register: u8, source_register: u8),
    OrRegisterToRegister(destination_register: u8, source_register: u8),
    AndRegisterToRegister(destination_register: u8, source_register: u8),
    XorRegisterToRegister(destination_register: u8, source_register: u8),
    AddRegisterToRegister(destination_register: u8, source_register: u8),
    SubtractRegiterFromRegister(destination_register: u8, source_register: u8),
    AddRegiterToRegister(destination_register: u8, source_register: u8),
    ShiftRight(register: u8),
    SubtractIntoDifferentRegister(source_register: u8, destination_register: u8),
    ShiftLeft(register: u8),
    SkipIfRegistersNotEqual(register_1: u8, register_2: u8),
    SetInstructionRegister(value: u16),
    JumpToLocationAndOffset0(address: u16),
    GenerateRandomData(register: u8, value: u16),
    DisplaySpriteAtLocation(x: u8, y: u8, n: u8),
    SkipIfPressedKeyEqualToRegister(register: u8),
    DontSkipIfPressedKeyEqualToRegister(register: u8),
    SetRegisterToDelayTimer(destination_register: u8),
    WaitForKeyPressAndStoreValue(register: u8),
    SetDelayTimerToRegister(source_register: u8),
    SetSoundTimerToRegister(source_register: u8),
    AddRegisterToIRegister(source_register: u8),
    SetIToFontAddress(digit: u8),
    StoreBCDValueOfRegisterToI(register: u8),
    StoreNRegistersToMemory(n: u8),
    ReadNRegistersFromMemory(n: u8)
}

impl Instruction {
    pub fn get_instruction_from_word(word: u16) -> Instruction {
        Instruction::Return
    }
}
