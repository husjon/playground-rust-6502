use std::usize;

pub struct Memory {
    pub memory: Vec<u8>,
    size: usize,
}

impl Memory {
    pub fn new(max_size: usize) -> Self {
        Self {
            memory: vec![0; max_size],
            size: max_size,
        }
    }

    pub fn reset(&self) {
        self::Memory::new(self.size);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize] as u8
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}
