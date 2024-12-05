use std::usize;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16; // array sizes have to be of size usize
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200; // 512 in decimal, which is the standard starting address for executables in chip8
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,
        };
        // load fonts into the first FONTSET_SIZE elements in ram
        // copy_from_slice ensures that both sides have the same size, otherwise it panics
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    fn push(&mut self, val: u16) {
        // indexing in rust requires usize type
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // fetch
        let op = self.fetch();
        // decode and execute
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        // since the ram is using u8 (byte) values, and the instruction is u16 (2 bytes)
        // we have to fetch two bytes at a time
        let higher_byte = self.ram[self.pc as usize] as u16; // ex: 0x12 --> 0x0012
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16; // ex : 0x34 --> 0x0034
        let op = (higher_byte << 8) | lower_byte; // (0x0012 << 8) --> 0x1200 | 0x0034 --> 0x1234
        self.pc += 2;
        op
    }

    // decode and execute
    fn execute(&mut self, op: u16) {
        // opcodes have 4 hex digits, to decode the opcode, we need to separate each digit
        let digit1 = (op & 0xF000) >> 12; // shift by 3 hex digits or 12 bits
        let digit2 = (op & 0x0F00) >> 8;  // shift by 2 hex digits or 8 bits
        let digit3 = (op & 0x00F0) >> 4;  // shift by 1 hex digits or 4 bits
        let digit4 = op & 0x000F;
        // match statement to sepcify match pattern for our opcode
        match (digit1, digit2, digit3, digit4) {
            // 0x0000 NOP
            (0, 0, 0, 0) => return,
            // 0x00E0 CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            // 0x00EE RET
            (0, 0, 0xE, 0xE) => {
                // pop the return address from the stack
                let ret_addr = self.pop();
                // put the the return address into PC
                self.pc = ret_addr;
            },
            // 0x1NNN JMP
            (1, _, _, _) => {
                // take the address NNN and put it in the PC
                let addr = op & 0xFFF;
                self.pc = addr;
            },
            // 0x2NNN CALL
            (2, _, _, _) => {
                // Take the address NNN from the opcode
                let addr = op & 0xFFF;
                // push the address into the stack
                self.push(self.pc);
                // put the address in PC
                self.pc = addr;
            },
            // 0x3XNN SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            },
            // 0x4XNN SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },
            // 0x5XY0 SKIP VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },
            // 0x6XNN VX = NN (similar to MOV)
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },
            // 0x7XNN VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                // using wrapping_add instead of += in case of integer overflow that might panic
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },
            // 0x8XY0 VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            },
            (_, _, _, _) => unimplemented!("unimplemented opcode {}", op)
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // TODO: BEEB
            }
            self.st -= 1;
        }
    }
}
