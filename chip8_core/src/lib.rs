use std::usize;
use rand::{random, seq::index};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16; // array sizes have to be of size usize
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200; // 512 in decimal, which is the standard starting address for executables in chip8
const FONTSET_SIZE: usize = 80;
const NUM_KEYS: usize = 16;

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
    keys: [bool; NUM_KEYS],
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
            keys: [false; NUM_KEYS],
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
            // 0x8XY1 VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },
            // 0x8XY2 VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            },
            // 0x8XY3 VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            },
            // 0x8XY4 VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                // check for carry, if there is carry set the carry flag in register VF
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry {1} else {0};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // 0x8XY5 VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;

            },
            // 0x8XY6 VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },
            // 0x8XY7 VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // 0x8XYE VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },
            // 0x9XY0 SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },
            // ANNN I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },
            // BNNN JMP to V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            },
            // CXNN  VX = rand() & NN
            (0XC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF);
                // have to specify u8 for random() to know which type is gonna be generated
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            },
            // DXYN Draw Sprite
            (0xD, _, _, _) => {
                // get the X and Y coordinates
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let x_coord = self.v_reg[digit3 as usize] as u16;
                // The last digit (N) determines how many rows higher is the sprite
                let num_rows = digit4;
                // flipped pixel tracking
                let mut flipped = false;
                // iterate over each row of the sprite
                for y_line in 0..num_rows {
                    // memory address of the sprite row data
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    // iterate over each column in the row, every row is 8 pixels wide
                    for x_line in 0..8 {
                        // fetch pixels using a mask
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // wrap around screen using modulo
                            let x = (x_coord + x_line) as uszie % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as uszie % SCREEN_WIDTH;

                            // get pixel index for the 1D screen array
                            let index = x + SCREEN_WIDTH * y;
                            // check flipping
                            flipped |= self.screen[index];
                            self.screen[index] ^= true;
                        }
                    }
                }
                // if the pixel flipped set VF regsiter
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },
            // EX9E Skip if key pressed
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                // if key pressed skip instruction
                if key {
                    self.pc += 2;
                }
            },
            // EXA1 Skip if key not pressed
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                // if key pressed skip instruction
                if !key {
                    self.pc += 2;
                }
            }, 
            // FX07 VX = DT
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            }
            // FX0A - Wait for Key Press
            (0xF, _, 0, 0xA) => {
                let x = digit2;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // repeat instruction if no key is pressed, to be stuck in a loop untill a
                    // key is pressed
                    self.pc -= 2;
                }

            },
            // FX15 - DT = VX
            (0xF, _, 1, 5) => {
                let x = digit2;
                self.dt = self.v_reg[x];
            },
            // FX18 - ST = VX
            (0xF, _, 1, 8) => {
                let x = digit2;
                self.st = self.v_reg[x];
            },
            // FX1E - I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2;
                let vx = self.v_reg[x];
                self.i_reg = self.i_reg.wrapping_add(vx);
            },
            // FX29 - Set I to Font Address
            (0xF, _, 2, 9) => {
                let x = digit2;
                let c = self.v_reg[x] as u16;
                // note that we stored fonts at the begginning of the RAM, and each font is 5
                // bytes so each character is stored at its index * 5 in RAM
                self.i_reg *= 5;
            },
            // FX33 - I = BCD of VX
            (0xF, _, 3, 3) => {
                let x = digit2;
                let vx = self.v_reg[x];
                // fetch each decimal
                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10).floor() as u8;
                let ones = (vx % 10.0) as u8;
                // store in ram
                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },
            // FX55 Store V0 -> VX into I
            (0xF, _, 5, 5) => {
                let x = digit2;
                let i = self.i_reg as usize;
                for index in 0..=x {
                    self.ram[i + index] = self.v_reg[index];
                }
            },
            // FX65 Load I into V0 -> VX
            (0xF, _, 6, 5) => {
                let x = digit2;
                let i = self.i_reg as usize;
                for index in 0..=x {
                    self.v_reg[index] = self.ram[i + index];
                }
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

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, index: usize, pressed: bool) {
        self.keys[index] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
}
