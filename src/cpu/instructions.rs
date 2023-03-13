use super::{AddressingMode, CpuFlags, Memory, CPU};

impl CPU {
    /// ADC - Add with Carry
    #[allow(clippy::cast_possible_truncation)]
    pub fn adc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        // convert u8 to u16 for easy carry bit logic
        let sum: u16 = u16::from(self.register_a)
            + u16::from(value) // add accmulator and value together; no worry if overflow because both are u8s
            + u16::from(self.status.contains(CpuFlags::CARRY)); // Add 1 carry bit was set

        // Should carry bit be set/removed
        if (sum > 0xFF) {
            self.status.insert(CpuFlags::CARRY);
        } else {
            self.status.remove(CpuFlags::CARRY);
        }

        // Truncate sum
        let result: u8 = sum as u8;

        // testing if sign bit is incorrect... somhow?
        // 0x80 is 1<<7 (0b1000_0000) aka is neg bit set?
        if (result ^ value) & (result ^ self.register_a) & 0x80 == 0 {
            self.status.remove(CpuFlags::OVERFLOW);
        } else {
            self.status.insert(CpuFlags::OVERFLOW);
        }

        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    /// AND - Logical AND
    pub fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// ASL - Arithmetic Shift Left
    ///
    /// Shifts all bits left one position. 0 is shifted into bit 0 and the
    /// original bit 7 is shifted into the Carry.
    pub fn asl(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.asl_accumulator();
        } else {
            let _ = self.asl_addr(mode);
        }
    }

    fn asl_accumulator(&mut self) {
        let mut data = self.register_a;

        if data >> 7 == 1 {
            self.status.insert(CpuFlags::CARRY);
        } else {
            self.status.remove(CpuFlags::CARRY);
        }

        data <<= 1;
        self.register_a = data;
        self.update_zero_and_negative_flags(data);
    }

    fn asl_addr(&mut self, mode: AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        if data >> 7 == 1 {
            self.status.insert(CpuFlags::CARRY);
        } else {
            self.status.remove(CpuFlags::CARRY);
        }

        data <<= 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
    }

    /// BCC - Branch if Carry Clear
    /// BCS - Branch if Carry Set
    /// BEQ - Branch if Equal
    /// BIT - Bit Test
    /// BMI - Branch if Minus
    /// BNE - Branch if Not Equal
    /// BPL - Branch if Positive
    /// BRK - Force Interrupt
    /// BVC - Branch if Overflow Clear
    /// BVS - Branch if Overflow Set
    /// CLC - Clear Carry Flag
    /// CLD - Clear Decimal Mode
    /// CLI - Clear Interrupt Disable
    /// CLV - Clear Overflow Flag
    /// CMP - Compare
    /// CPX - Compare X Register
    /// CPY - Compare Y Register
    /// DEC - Decrement Memory
    /// DEX - Decrement X Register
    /// DEY - Decrement Y Register
    /// EOR - Exclusive OR
    /// INC - Increment Memory
    /// INX - Increment X Register
    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// INY - Increment Y Register
    pub fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// JMP - Jump
    /// JSR - Jump to Subroutine
    /// LDA - Load Accumulator
    pub fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// LDX - Load X Register
    pub fn ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// LDY - Load Y Register
    pub fn ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// LSR - Logical Shift Right
    /// NOP - No Operation
    /// ORA - Logical Inclusive OR
    /// PHA - Push Accumulator
    /// PHP - Push Processor Status
    /// PLA - Pull Accumulator
    /// PLP - Pull Processor Status
    /// ROL - Rotate Left
    /// ROR - Rotate Right
    /// RTI - Return from Interrupt
    /// RTS - Return from Subroutine
    /// SBC - Subtract with Carry
    /// SEC - Set Carry Flag
    /// SED - Set Decimal Flag
    /// SEI - Set Interrupt Disable
    /// STA - Store Accumulator
    pub fn sta(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    /// STX - Store X Register
    pub fn stx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    /// STY - Store Y Register
    pub fn sty(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    /// TAX - Transfer Accumulator to X
    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// TAY - Transfer Accumulator to Y
    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// TSX - Transfer Stack Pointer to X
    /// TXA - Transfer X to Accumulator
    pub fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// TXS - Transfer X to Stack Pointer
    /// TYA - Transfer Y to Accumulator
    pub fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
}
