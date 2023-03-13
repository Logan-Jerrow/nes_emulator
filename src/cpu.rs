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

use self::cpuflags::CpuFlags;
use self::memory::Memory;
use crate::addressing_mode::AddressingMode;
use crate::opcode::{mnemonic::Mnemonic, OpCode};

mod cpuflags;
mod instructions;
mod memory;

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuFlags,
    pub program_counter: u16,
    pub stack_ptr: u8,
    memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            stack_ptr: Self::STACK_RESET,
            status: CpuFlags::default(),
            memory: [0; 0xFFFF],
        }
    }
}

impl Memory for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    // [0x8000 .. 0xFFFF] Program ROM (PRG ROM)
    const PRG_ROM_START_ADDR: u16 = 0x8000;
    const PRG_ROM_EXEC_ADDR: u16 = 0xfffc;

    // Stack Pointer - Memory space [0x0100 .. 0x01FF] is used for stack.
    const STACK_MEMORY_START: u16 = 0x0100;
    const STACK_MEMORY_END: u16 = 0x01ff;

    // https://archive.nes.science/nesdev-forums/f3/t715.xhtml#p7591
    // by WedNESday on 2005-12-21 (#7591)
    // It doesn't matter where it starts as it wraps anyway and all programmers are aware of that.
    // The NES may set it to 0xFD on power-up/reset (I wasn't aware of that until now) but don't
    // worry about it. Most emulators of the 6502 set it to 0xFF. Just make sure that your stack
    // pointer is 8-bit and works something like this;
    //
    // CPU.Memory[Stack + 0x100] = ...
    //
    //
    // https://old.reddit.com/r/EmuDev/comments/g8ky04/6502_startreset_sequence_and_nestest/
    //
    // It started at zero. As part of the reset process the CPU decremented S three times. By the
    // time the first program instruction is executed S is $FD (0 minus 3).
    const STACK_RESET: u8 = 0xfd; // 0 - 3 = 0xfd (Wrapping!)

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result >> 7 == 1 {
            self.status.insert(CpuFlags::NEGATIV);
        } else {
            self.status.remove(CpuFlags::NEGATIV);
        }
    }

    pub fn reset(&mut self) {
        *self = Self {
            program_counter: self.mem_read_u16(Self::PRG_ROM_EXEC_ADDR),
            memory: self.memory,
            ..Default::default()
        };
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: &[u8]) {
        let start: usize = Self::PRG_ROM_START_ADDR.into();
        self.memory[start..(start + program.len())].copy_from_slice(program);
        self.mem_write_u16(Self::PRG_ROM_EXEC_ADDR, Self::PRG_ROM_START_ADDR);
    }

    fn get_operand_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter).into(),
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x).into()
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y).into()
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x.into())
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y.into())
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr.into());
                let hi = self.mem_read(ptr.wrapping_add(1).into());

                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base.into());
                let hi = self.mem_read(base.wrapping_add(1).into());
                let deref_base = u16::from_le_bytes([lo, hi]);

                deref_base.wrapping_add(self.register_y.into())
            }
            AddressingMode::Implicit
            | AddressingMode::Accumulator
            | AddressingMode::Relative
            | AddressingMode::Indirect => panic!("mode {mode:?} is not supported."),
        }
    }

    pub fn run(&mut self) {
        loop {
            let raw_opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = OpCode::decode(raw_opcode);
            match opcode.mnemonic {
                Mnemonic::Adc => self.adc(opcode.mode),
                Mnemonic::And => self.and(opcode.mode),
                Mnemonic::Asl => self.asl(opcode.mode),
                Mnemonic::Bcc => todo!(),
                Mnemonic::Bcs => todo!(),
                Mnemonic::Beq => todo!(),
                Mnemonic::Bit => todo!(),
                Mnemonic::Bmi => todo!(),
                Mnemonic::Bne => todo!(),
                Mnemonic::Bpl => todo!(),
                Mnemonic::Brk => return,
                Mnemonic::Bvc => todo!(),
                Mnemonic::Bvs => todo!(),
                Mnemonic::Clc => todo!(),
                Mnemonic::Cld => todo!(),
                Mnemonic::Cli => todo!(),
                Mnemonic::Clv => todo!(),
                Mnemonic::Cmp => todo!(),
                Mnemonic::Cpx => todo!(),
                Mnemonic::Cpy => todo!(),
                Mnemonic::Dec => todo!(),
                Mnemonic::Dex => todo!(),
                Mnemonic::Dey => todo!(),
                Mnemonic::Eor => todo!(),
                Mnemonic::Inc => todo!(),
                Mnemonic::Inx => self.inx(),
                Mnemonic::Iny => self.iny(),
                Mnemonic::Jmp => todo!(),
                Mnemonic::Jsr => todo!(),
                Mnemonic::Lda => self.lda(opcode.mode),
                Mnemonic::Ldx => self.ldx(opcode.mode),
                Mnemonic::Ldy => self.ldy(opcode.mode),
                Mnemonic::Lsr => todo!(),
                Mnemonic::Nop => todo!(),
                Mnemonic::Ora => todo!(),
                Mnemonic::Pha => todo!(),
                Mnemonic::Php => todo!(),
                Mnemonic::Pla => todo!(),
                Mnemonic::Plp => todo!(),
                Mnemonic::Rol => todo!(),
                Mnemonic::Ror => todo!(),
                Mnemonic::Rti => todo!(),
                Mnemonic::Rts => todo!(),
                Mnemonic::Sbc => todo!(),
                Mnemonic::Sec => todo!(),
                Mnemonic::Sed => todo!(),
                Mnemonic::Sei => todo!(),
                Mnemonic::Sta => self.sta(opcode.mode),
                Mnemonic::Stx => self.stx(opcode.mode),
                Mnemonic::Sty => self.sty(opcode.mode),
                Mnemonic::Tax => self.tax(),
                Mnemonic::Tay => self.tay(),
                Mnemonic::Tsx => todo!(),
                Mnemonic::Txa => self.txa(),
                Mnemonic::Txs => todo!(),
                Mnemonic::Tya => self.tya(),
            }

            if program_counter_state == self.program_counter {
                // minus one since we inc when mem_read @ start of fn
                self.program_counter += u16::from(opcode.len - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.status.bits() & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::default();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xA5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xA9, 0x00, 0x00]);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xA9, 0x0A, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0xC1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }
}
