use crate::memory::Memory;
use crate::input::Input;
use crate::graphics::Graphics;
use crate::sound::Sound;

pub struct Bus {
    pub memory: Memory,

    pub input: Input,

    pub graphics: Graphics,

    pub sound: Sound,
}

impl Bus {

    pub fn mock() -> Bus {
        Bus {
            memory: Memory::mock(),
            input: Input::mock(),
            graphics: Graphics::mock(),
            sound: Sound::mock()
        }
    }
    pub fn new() -> Bus {
        Bus {
            memory: Memory::new(),
            input: Input::new(),
            graphics: Graphics::new(),
            sound: Sound::mock(),
        }
    }
}