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
    pub fn new(memory: Memory) -> Bus {
        Bus {
            memory: memory,
            input: Input::new(),
            graphics: Graphics::new(),
            sound: Sound::new(),
        }
    }
}