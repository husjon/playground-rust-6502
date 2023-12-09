use crate::memory::Memory;

enum Stage {
    Fetch,
    Decode,
    Execute,
}

pub struct CPU {
    pub pc: u16, // Program Counter
    pub sp: u8,  // Stack Pointer

    pub a: u8, // Accumulator
    pub x: u8, // Register X
    pub y: u8, // Register Y

    pub ps: Flags, // Processor Status

    pub memory: Memory,

    // remaining_cycles: u8,
    next_stage: Stage,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flags {
    c: bool, // Carry
    z: bool, // Zero
    i: bool, // Interrupt Disable
    d: bool, // Decimal Mode
    b: bool, // Break Command
    v: bool, // Overflow
    n: bool, // Negative
}
