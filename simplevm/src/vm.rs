enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    FLAGS,
}

struct Machine {
    registers: [u16; 7],
    memory: [u8; 5000],
}
