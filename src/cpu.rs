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

use self::{cpuflags::CpuFlags, memory::Memory};
use crate::{
    addressing_mode::AddressingMode,
    opcode::{self, mnemonic::Mnemonic, OpCode},
};

pub mod memory;

mod cpuflags;
mod instructions;
mod opcode_array;

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
const STACK_RESET: u8 = 0xFD; // 0 - 3 = 0xfd (Wrapping!)

// Stack Pointer - Memory space [0x0100 .. 0x01FF] is used for stack.
const STACK_START: u16 = 0x0100;
const STACK_MEMORY_END: u16 = 0x01FF;

// [0x8000 .. 0xFFFF] Program ROM (PRG ROM)
const PRG_ROM_START_ADDR: u16 = 0x0600;
const PRG_ROM_EXEC_ADDR: u16 = 0xFFFC;

#[derive(Debug)]
pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    status: CpuFlags,
    program_counter: u16,
    stack_ptr: u8,
    memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            stack_ptr: STACK_RESET,
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
    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Self),
    {
        loop {
            let raw_opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcode_array::decode(raw_opcode);
            match opcode.mnemonic {
                Mnemonic::Adc => self.adc(opcode.mode),
                Mnemonic::And => self.and(opcode.mode),
                Mnemonic::Asl => self.asl(opcode.mode),
                Mnemonic::Bcc => self.bcc(),
                Mnemonic::Bcs => self.bcs(),
                Mnemonic::Beq => self.beq(),
                Mnemonic::Bit => self.bit(opcode.mode),
                Mnemonic::Bmi => self.bmi(),
                Mnemonic::Bne => self.bne(),
                Mnemonic::Bpl => self.bpl(),
                Mnemonic::Brk => return,
                Mnemonic::Bvc => self.bvc(),
                Mnemonic::Bvs => self.bvs(),
                Mnemonic::Clc => self.clc(),
                Mnemonic::Cld => self.cld(),
                Mnemonic::Cli => self.cli(),
                Mnemonic::Clv => self.clv(),
                Mnemonic::Cmp => self.compare(opcode.mode, self.register_a),
                Mnemonic::Cpx => self.compare(opcode.mode, self.register_x),
                Mnemonic::Cpy => self.compare(opcode.mode, self.register_y),
                Mnemonic::Dec => self.dec(opcode.mode),
                Mnemonic::Dex => self.dex(opcode.mode),
                Mnemonic::Dey => self.dey(opcode.mode),
                Mnemonic::Eor => self.eor(opcode.mode),
                Mnemonic::Inc => self.inc(opcode.mode),
                Mnemonic::Inx => self.inx(),
                Mnemonic::Iny => self.iny(),
                Mnemonic::Jmp => self.jmp(opcode.mode),
                Mnemonic::Jsr => self.jsr(),
                Mnemonic::Lda => self.lda(opcode.mode),
                Mnemonic::Ldx => self.ldx(opcode.mode),
                Mnemonic::Ldy => self.ldy(opcode.mode),
                Mnemonic::Lsr => self.lsr(opcode.mode),
                Mnemonic::Nop => (),
                Mnemonic::Ora => self.ora(opcode.mode),
                Mnemonic::Pha => self.pha(opcode.mode),
                Mnemonic::Php => self.php(opcode.mode),
                Mnemonic::Pla => self.pla(opcode.mode),
                Mnemonic::Plp => self.plp(opcode.mode),
                Mnemonic::Rol => self.rol(opcode.mode),
                Mnemonic::Ror => self.ror(opcode.mode),
                Mnemonic::Rti => self.rti(),
                Mnemonic::Rts => self.rts(),
                Mnemonic::Sbc => self.sbc(opcode.mode),
                Mnemonic::Sec => self.sec(),
                Mnemonic::Sed => self.sed(),
                Mnemonic::Sei => self.sei(),
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

            callback(self);
        }
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn load(&mut self, program: &[u8]) {
        let start: usize = PRG_ROM_START_ADDR.into();
        self.memory[start..(start + program.len())].copy_from_slice(program);
        self.mem_write_u16(PRG_ROM_EXEC_ADDR, PRG_ROM_START_ADDR);
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_ptr = STACK_RESET;
        self.status = CpuFlags::default();
        // memory: [0; 0xFFFF],

        self.program_counter = self.mem_read_u16(PRG_ROM_EXEC_ADDR);
    }

    // Stack impl
    pub fn stack_pop(&mut self) -> u8 {
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
        self.mem_read(STACK_START + u16::from(self.stack_ptr))
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK_START + u16::from(self.stack_ptr), data);
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop();
        let high = self.stack_pop();
        u16::from_le_bytes([low, high])
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        let [low, high] = data.to_le_bytes();
        self.stack_push(high);
        self.stack_push(low);
    }

    // utility fn
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

    fn get_memory(&mut self, mode: AddressingMode) -> (u16, u8) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        (addr, value)
    }

    fn set_accumulator(&mut self, data: u8) {
        self.register_a = data;
        self.update_zero_and_negative_flags(data);
    }

    fn set_memory(&mut self, addr: u16, data: u8) {
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn update_zero_flag(&mut self, result: u8) {
        self.status.set(CpuFlags::ZERO, result == 0);
    }

    fn update_negative_flag(&mut self, result: u8) {
        self.status.set(CpuFlags::NEGATIV, result >> 7 == 1);
    }

    /// msb = bit 7 for u8
    fn msb_to_carry_flag(&mut self, value: u8) {
        self.status.set(CpuFlags::CARRY, value >> 7 == 1);
    }

    fn lsb_to_carry_flag(&mut self, value: u8) {
        self.status.set(CpuFlags::CARRY, value & 1 == 1);
    }

    #[allow(
        clippy::cast_lossless,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn branch(&mut self, condition: bool) {
        if condition {
            let data = self.mem_read(self.program_counter);
            let data = i8::from_le_bytes([data]);
            let data = i16::from(data);

            self.program_counter = self
                .program_counter
                // program counter increment durring instruction execution
                .wrapping_add(1)
                .wrapping_add_signed(data);
        }
    }

    fn compare(&mut self, mode: AddressingMode, with: u8) {
        let (addr, data) = self.get_memory(mode);
        self.status.set(CpuFlags::CARRY, with >= data);
        self.update_zero_and_negative_flags(with.wrapping_sub(data));
    }

    fn add_to_accumulator(&mut self, data: u8) {
        // convert u8 to u16 for easy carry bit logic
        let sum: u16 = u16::from(self.register_a)
            + u16::from(data) // add accmulator and value together; no worry if overflow because both are u8s
            + u16::from(self.status.contains(CpuFlags::CARRY)); // Add 1 if carry bit was set

        self.status.set(CpuFlags::CARRY, sum > u8::MAX.into());

        // Truncate sum
        // let result: u8 = sum as u8;
        let [result, _]: [u8; 2] = sum.to_le_bytes(); // no 'as' keyword

        // testing if sign bit is incorrect... somhow?
        // 0x80 is 1<<7 (0b1000_0000) aka is neg bit set?
        let msb = 1 << 7;
        let pred = (result ^ data) & (result ^ self.register_a) & msb != 0;
        self.status.set(CpuFlags::OVERFLOW, pred);

        self.set_accumulator(result);
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

    #[test]
    fn msb_carry() {
        let mut cpu = CPU::default();
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        cpu.msb_to_carry_flag(0b1000_0000);
        assert!(cpu.status.contains(CpuFlags::CARRY));
    }
}
