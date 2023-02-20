// RAM accessible via [0x0000 .. 0x2000] address space.
// [0x2000 .. 0x4020] redirected to other nes modules: PPU, APU, Gamepades, etc.
// [0x4020 .. 0x6000] cartridges defined. Ignore
// [0x6000 .. 0x8000] RAM space. Ignore

// NES CPU 7 Registers
// Program Counter (PC) - holds the address for the next machine language instruction to be
// executed.

// Stack Pointer - Memory space [0x0100 .. 0x1FF] is used for stack. The stack pointer holds the
// address of the top of that space. NES Stack (as all stacks) grows from top to bottom: when a
// byte gets pushed to the stack, SP register decrements. When a byte is retrieved from the stack,
// SP register increments.

// Accumulator (A) - stores the results of arithmetic, logic, and memory access operations. It used
// as an input parameter for some operations.
// Index Register X (X) - used as an offset in specific memory addressing modes (more on this
// later). Can be used for auxiliary storage needs (holding temp values, being used as a counter,
// etc.)

// Index Register Y (Y) - similar use cases as register X.

// Processor status (P) - 8-bit register represents 7 status flags that can be set or unset
// depending on the result of the last executed instruction (for example Z flag is set (1) if the
// result of an operation is 0, and is unset/erased (0) otherwise)

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: Default::default(),
            register_x: Default::default(),
            status: Default::default(),
            program_counter: Default::default(),
            memory: [0; 0xFFFF],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

impl CPU {
    // [0x8000 .. 0xFFFF] Program ROM (PRG ROM)
    const PRG_ROM_START_ADDR: u16 = 0x8000;
    const PRG_ROM_EXEC_ADDR: u16 = 0xFFFC;

    pub fn new() -> Self {
        Self::default()
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = Default::default();
        self.register_x = Default::default();
        self.status = Default::default();

        self.program_counter = self.mem_read_u16(Self::PRG_ROM_EXEC_ADDR);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let start: usize = Self::PRG_ROM_START_ADDR.into();
        self.memory[start..(start + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(Self::PRG_ROM_EXEC_ADDR, Self::PRG_ROM_START_ADDR);
    }

    pub fn run(&mut self) {
        loop {
            let ops_code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match ops_code {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let parameter = self.mem_read(self.program_counter);
                    self.program_counter += 1;

                    self.lda(parameter);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,

                _ => todo!(),
            }
        }
        // Fetch next execution instruction from the instruction memory

        // Decode the instruction
        // Execute the Instruction
        // Repeat the cycle
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
