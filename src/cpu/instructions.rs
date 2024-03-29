use super::{AddressingMode, CpuFlags, Memory, CPU};

impl CPU {
    /// ADC - Add with Carry
    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn adc(&mut self, mode: AddressingMode) {
        let (_, data) = self.get_memory(mode);
        self.add_to_accumulator(data);
    }

    /// AND - Logical AND
    pub(super) fn and(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);
        self.set_accumulator(self.register_a & data);
    }

    /// ASL - Arithmetic Shift Left
    ///
    /// Shifts all bits left one position. 0 is shifted into bit 0 and the
    /// original bit 7 is shifted into the Carry.
    pub(super) fn asl(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.asl_accumulator();
        } else {
            let _ = self.asl_addr(mode);
        }
    }

    fn asl_accumulator(&mut self) {
        let mut data = self.register_a;

        self.msb_to_carry_flag(data);

        self.set_accumulator(data << 1);
    }

    fn asl_addr(&mut self, mode: AddressingMode) -> u8 {
        let (addr, mut data) = self.get_memory(mode);

        self.msb_to_carry_flag(data);

        data <<= 1;
        self.set_memory(addr, data);
        data
    }

    /// BCC - Branch if Carry Clear
    ///
    /// If the carry flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub(super) fn bcc(&mut self) {
        self.branch(!self.status.contains(CpuFlags::CARRY));
    }

    /// BCS - Branch if Carry Set
    pub(super) fn bcs(&mut self) {
        self.branch(self.status.contains(CpuFlags::CARRY));
    }

    /// BEQ - Branch if Equal
    pub(super) fn beq(&mut self) {
        self.branch(self.status.contains(CpuFlags::ZERO));
    }

    /// BIT - Bit Test
    ///
    /// This instructions is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is AND with the value in memory to set or clear the zero flag, but
    /// the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V
    /// flags.
    pub(super) fn bit(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);

        let result = self.register_a & data;
        self.update_zero_flag(result);

        self.status
            .set(CpuFlags::OVERFLOW, data & CpuFlags::OVERFLOW.bits() > 0);
        self.status
            .set(CpuFlags::NEGATIV, data & CpuFlags::NEGATIV.bits() > 0);
    }

    /// BMI - Branch if Minus
    pub(super) fn bmi(&mut self) {
        self.branch(self.status.contains(CpuFlags::NEGATIV));
    }

    /// BNE - Branch if Not Equal
    pub(super) fn bne(&mut self) {
        self.branch(!self.status.contains(CpuFlags::ZERO));
    }

    /// BPL - Branch if Positive
    pub(super) fn bpl(&mut self) {
        self.branch(!self.status.contains(CpuFlags::NEGATIV));
    }

    /// BRK - Force Interrupt
    /// BVC - Branch if Overflow Clear
    pub(super) fn bvc(&mut self) {
        self.branch(!self.status.contains(CpuFlags::OVERFLOW));
    }

    /// BVS - Branch if Overflow Set
    pub(super) fn bvs(&mut self) {
        self.branch(self.status.contains(CpuFlags::OVERFLOW));
    }

    /// CLC - Clear Carry Flag
    pub(super) fn clc(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }

    /// CLD - Clear Decimal Mode
    pub(super) fn cld(&mut self) {
        self.status.remove(CpuFlags::DECIMAL_MODE);
    }

    /// CLI - Clear Interrupt Disable
    pub(super) fn cli(&mut self) {
        self.status.remove(CpuFlags::INTERUPT_DISABLE);
    }

    /// CLV - Clear Overflow Flag
    pub(super) fn clv(&mut self) {
        self.status.remove(CpuFlags::OVERFLOW);
    }

    // CMP - Compare
    // CPX - Compare X Register
    // CPY - Compare Y Register
    /// DEC - Decrement Memory
    pub(super) fn dec(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);
        self.set_memory(addr, data.wrapping_sub(1));
    }

    /// DEX - Decrement X Register
    pub(super) fn dex(&mut self, mode: AddressingMode) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// DEY - Decrement Y Register
    pub(super) fn dey(&mut self, mode: AddressingMode) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// EOR - Exclusive OR
    pub(super) fn eor(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);
        self.set_accumulator(self.register_a ^ data);
    }

    /// INC - Increment Memory
    pub(super) fn inc(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);
        self.set_memory(addr, data.wrapping_add(1));
    }

    /// INX - Increment X Register
    pub(super) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// INY - Increment Y Register
    pub fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// JMP - Jump
    pub fn jmp(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Absolute {
            self.program_counter = self.mem_read_u16(self.program_counter);
        }
        if mode == AddressingMode::Indirect {
            let addr = self.mem_read_u16(self.program_counter);
            // An original 6502 has does not correctly fetch the target address if the indirect
            // vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF).
            let is_page_boundary = addr & 0x00FF == 0x00FF;
            let indirect_addr = if is_page_boundary {
                // In this case fetches the LSB from $xxFF as expected
                let low = self.mem_read(addr);
                // but takes the MSB from $xx00.
                let high = self.mem_read(addr & 0xFF00);
                u16::from_le_bytes([low, high])
            } else {
                self.mem_read_u16(addr)
            };

            self.program_counter = indirect_addr;
        }
    }

    /// JSR - Jump to Subroutine
    /// The JSR instruction pushes the address (minus one) of the return point on to the stack and
    /// then sets the program counter to the target memory address.
    pub(super) fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_addr = self.mem_read_u16(self.program_counter);
        self.program_counter = target_addr;
    }
    /// LDA - Load Accumulator
    pub(super) fn lda(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);
        self.set_accumulator(data);
    }

    /// LDX - Load X Register
    pub(super) fn ldx(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);

        self.register_x = data;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// LDY - Load Y Register
    pub(super) fn ldy(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);

        self.register_y = data;
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// LSR - Logical Shift Right
    pub(super) fn lsr(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.lsr_accumulator();
        } else {
            let _ = self.lsr_addr(mode);
        }
    }

    fn lsr_accumulator(&mut self) {
        let mut data = self.register_a;

        self.lsb_to_carry_flag(data);

        self.set_accumulator(data >> 1);
    }

    fn lsr_addr(&mut self, mode: AddressingMode) -> u8 {
        let (addr, mut data) = self.get_memory(mode);

        self.lsb_to_carry_flag(data);

        data >>= 1;
        self.set_memory(addr, data);
        data
    }

    /// NOP - No Operation
    pub(super) const fn nop() {}
    /// ORA - Logical Inclusive OR
    pub(super) fn ora(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_memory(mode);
        self.set_accumulator(self.register_a | data);
    }

    /// PHA - Push Accumulator
    pub(super) fn pha(&mut self, mode: AddressingMode) {
        self.stack_push(self.register_a);
    }

    /// PHP - Push Processor Status
    pub(super) fn php(&mut self, mode: AddressingMode) {
        self.stack_push(self.status.bits());
    }

    /// PLA - Pull Accumulator
    pub(super) fn pla(&mut self, mode: AddressingMode) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// PLP - Pull Processor Status
    pub(super) fn plp(&mut self, mode: AddressingMode) {
        self.status = CpuFlags::from_bits_truncate(self.stack_pop());
    }

    /// ROL - Rotate Left
    pub(super) fn rol(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.rol_accumulator();
        } else {
            self.rol_memory(mode);
        }
    }
    fn rol_accumulator(&mut self) {
        let mut data = self.register_a;
        let carry = self.status.contains(CpuFlags::CARRY);
        self.msb_to_carry_flag(data);
        data <<= 1;
        if carry {
            data |= 1;
        }

        self.set_accumulator(data);
    }

    fn rol_memory(&mut self, mode: AddressingMode) {
        let (addr, mut data) = self.get_memory(mode);
        let carry = self.status.contains(CpuFlags::CARRY);
        self.msb_to_carry_flag(data);
        data <<= 1;
        if carry {
            data |= 1;
        }

        self.set_memory(addr, data);
    }

    /// ROR - Rotate Right
    pub(super) fn ror(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.rol_accumulator();
        } else {
            self.rol_memory(mode);
        }
    }
    fn ror_accumulator(&mut self) {
        let mut data = self.register_a;
        let carry = self.status.contains(CpuFlags::CARRY);
        self.lsb_to_carry_flag(data);
        data >>= 1;
        if carry {
            data |= (1 << 7);
        }

        self.set_accumulator(data);
    }

    fn ror_memory(&mut self, mode: AddressingMode) {
        let (addr, mut data) = self.get_memory(mode);
        let carry = self.status.contains(CpuFlags::CARRY);
        self.lsb_to_carry_flag(data);
        data >>= 1;
        if carry {
            data |= (1 << 7);
        }

        self.set_memory(addr, data);
    }

    /// RTI - Return from Interrupt
    pub(super) fn rti(&mut self) {
        self.status = CpuFlags::from_bits_truncate(self.stack_pop());
        self.status.remove(CpuFlags::BREAK);
        self.status.insert(CpuFlags::BREAK2);

        self.program_counter = self.stack_pop_u16();
    }

    /// RTS - Return from Subroutine
    pub(super) fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    /// SBC - Subtract with Carry
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub(super) fn sbc(&mut self, mode: AddressingMode) {
        let (_, data) = self.get_memory(mode);
        let data = i8::from_le_bytes([data]);
        let data = (data).wrapping_neg().wrapping_sub(1);
        let [data] = i8::to_le_bytes(data);
        self.add_to_accumulator(data);
    }

    /// SEC - Set Carry Flag
    pub(super) fn sec(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }

    /// SED - Set Decimal Flag
    pub(super) fn sed(&mut self) {
        self.status.insert(CpuFlags::DECIMAL_MODE);
    }

    /// SEI - Set Interrupt Disable
    pub(super) fn sei(&mut self) {
        self.status.insert(CpuFlags::INTERUPT_DISABLE);
    }

    /// STA - Store Accumulator
    pub(super) fn sta(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    /// STX - Store X Register
    pub(super) fn stx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    /// STY - Store Y Register
    pub(super) fn sty(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    /// TAX - Transfer Accumulator to X
    pub(super) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// TAY - Transfer Accumulator to Y
    pub(super) fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// TSX - Transfer Stack Pointer to X
    /// TXA - Transfer X to Accumulator
    pub(super) fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// TXS - Transfer X to Stack Pointer
    /// TYA - Transfer Y to Accumulator
    pub(super) fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
}
