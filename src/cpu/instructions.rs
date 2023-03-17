use super::{AddressingMode, CpuFlags, Memory, CPU};

impl CPU {
    /// ADC - Add with Carry
    #[allow(clippy::cast_possible_truncation)]
    pub fn adc(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);

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

    /// AND - Logical AND
    pub fn and(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);
        self.set_accumulator(self.register_a & data);
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

        self.msb_to_carry_flag(data);

        self.set_accumulator(data << 1);
    }

    fn asl_addr(&mut self, mode: AddressingMode) -> u8 {
        let (addr, mut data) = self.get_data(mode);

        self.msb_to_carry_flag(data);

        data <<= 1;
        self.set_mem(addr, data);
        data
    }

    /// BCC - Branch if Carry Clear
    /// BCS - Branch if Carry Set
    /// BEQ - Branch if Equal

    /// BIT - Bit Test
    ///
    /// This instructions is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is AND with the value in memory to set or clear the zero flag, but
    /// the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V
    /// flags.
    pub fn bit(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);

        let result = self.register_a & data;
        self.update_zero_flag(result);

        self.status
            .set(CpuFlags::OVERFLOW, data & CpuFlags::OVERFLOW.bits() > 0);
        self.status
            .set(CpuFlags::NEGATIV, data & CpuFlags::NEGATIV.bits() > 0);
    }

    /// BMI - Branch if Minus
    /// BNE - Branch if Not Equal
    /// BPL - Branch if Positive
    /// BRK - Force Interrupt
    /// BVC - Branch if Overflow Clear
    /// BVS - Branch if Overflow Set
    /// CLC - Clear Carry Flag
    pub fn clc(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }

    /// CLD - Clear Decimal Mode
    pub fn cld(&mut self) {
        self.status.remove(CpuFlags::DECIMAL_MODE);
    }

    /// CLI - Clear Interrupt Disable
    pub fn cli(&mut self) {
        self.status.remove(CpuFlags::INTERUPT_DISABLE);
    }

    /// CLV - Clear Overflow Flag
    pub fn clv(&mut self) {
        self.status.remove(CpuFlags::OVERFLOW);
    }

    /// CMP - Compare
    /// CPX - Compare X Register
    /// CPY - Compare Y Register
    /// DEC - Decrement Memory
    pub fn dec(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);
        self.set_mem(addr, data.wrapping_sub(1));
    }

    /// DEX - Decrement X Register
    pub fn dex(&mut self, mode: AddressingMode) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// DEY - Decrement Y Register
    pub fn dey(&mut self, mode: AddressingMode) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// EOR - Exclusive OR
    pub fn eor(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);
        self.set_accumulator(self.register_a ^ data);
    }

    /// INC - Increment Memory
    pub fn inc(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);
        self.set_mem(addr, data.wrapping_add(1));
    }

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
    pub fn jsr(&mut self) {
        todo!()
    }
    /// LDA - Load Accumulator
    pub fn lda(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);

        self.register_a = data;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// LDX - Load X Register
    pub fn ldx(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);

        self.register_x = data;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// LDY - Load Y Register
    pub fn ldy(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);

        self.register_y = data;
        self.update_zero_and_negative_flags(self.register_y);
    }

    /// LSR - Logical Shift Right
    /// NOP - No Operation
    pub const fn nop() {}
    /// ORA - Logical Inclusive OR
    pub fn ora(&mut self, mode: AddressingMode) {
        let (addr, data) = self.get_data(mode);
        self.set_accumulator(self.register_a | data);
    }

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
    pub fn sec(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }

    /// SED - Set Decimal Flag
    pub fn sed(&mut self) {
        self.status.insert(CpuFlags::DECIMAL_MODE);
    }

    /// SEI - Set Interrupt Disable
    pub fn sei(&mut self) {
        self.status.insert(CpuFlags::INTERUPT_DISABLE);
    }

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
