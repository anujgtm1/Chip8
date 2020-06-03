struct Cpu {
    /// 4KB of memory
    /// 16 general purpose registers of 8 bit each
    /// 16 levels of stack 
    /// I -> Generally used to store memory addresses
    /// PC -> Program Counter
    /// S
    /// 
    pub registers: [u8; 8],             // 16 8 bit general purpose registers
    pub stack: [u16; 16],               // 16 levels of stack for function calls
    pub I : u16,                        // Special register used to store addresses
    pub PC: u16,                        // Program Counter
    pub SP: u8,                         // Stack Pointer
    pub DT: u8,                         // Delay Timer (Automatically decremented at a rate of 60Hz if set)
    pub ST: u8,                         // Sound Timer (Automatically decremented at a rate of 60Hz if set) Buzzer will soudn if ST > 0
}

