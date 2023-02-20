// RAM accessible via [0x0000 .. 0x2000] address space.
// [0x2000 .. 0x4020] redirected to other nes modules: PPU, APU, Gamepades, etc.
// [0x4020 .. 0x6000] cartridges defined. Ignore
// [0x6000 .. 0x8000] RAM space. Ignore
// [0x8000 .. 0x10000] Program ROM (PRG ROM)

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

use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
enum OpCodes {
    Lda = 0xA9,
    Tax = 0xAA,
    Inx = 0xE8,
    Brk = 0x00,
}

#[derive(Debug, Default)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let ops_code = OpCodes::try_from(program[self.program_counter as usize]);
            self.program_counter += 1;

            match ops_code {
                Ok(OpCodes::Lda) => {
                    let parameter = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(parameter);
                }
                Ok(OpCodes::Tax) => self.tax(),
                Ok(OpCodes::Inx) => self.inx(),
                Ok(OpCodes::Brk) => return,

                Err(_) => todo!(),
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
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
