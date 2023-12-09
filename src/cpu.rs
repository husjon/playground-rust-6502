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

pub struct Flags {
    c: bool, // Carry
    z: bool, // Zero
    i: bool, // Interrupt Disable
    d: bool, // Decimal Mode
    b: bool, // Break Command
    v: bool, // Overflow
    n: bool, // Negative
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x0,
            sp: 0x0,
            a: 0x0,
            x: 0x0,
            y: 0x0,

            ps: Flags {
                c: false,
                z: false,
                i: false,
                d: false,
                b: false,
                v: false,
                n: false,
            },

            memory: Memory::new(64 * 1024),
            // remaining_cycles: 0,
            next_stage: Stage::Fetch,
        }
    }
}
