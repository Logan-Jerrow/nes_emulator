use crate::{
    addressing_mode::AddressingMode,
    opcode::{self, mnemonic::Mnemonic, OpCode},
};

#[must_use]
pub fn decode(raw: opcode::Raw) -> OpCode {
    INSTRUCTIONS[usize::from(raw)].unwrap_or_else(|| panic!("OpCode {raw:#04x} is not recognized."))
}

const LEN: usize = 0xFF;
const INSTRUCTIONS: [Option<OpCode>; LEN] = padded_array();

const fn padded_array() -> [Option<OpCode>; LEN] {
    let mut array = [None; LEN];

    let mut index: usize = 0;
    while index < INSTRUCTION_ARRAY.len() {
        let entry: OpCode = INSTRUCTION_ARRAY[index];
        array[entry.code as usize] = Some(entry);
        index += 1;
    }

    array
}

const INSTRUCTION_ARRAY: [OpCode; 151] = [
    // ADC - Add with Carry
    (OpCode::new(0x69, Mnemonic::Adc, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0x65, Mnemonic::Adc, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x75, Mnemonic::Adc, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x6D, Mnemonic::Adc, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0x7D, Mnemonic::Adc, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0x79, Mnemonic::Adc, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0x61, Mnemonic::Adc, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0x71, Mnemonic::Adc, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // AND - Logical AND
    (OpCode::new(0x29, Mnemonic::And, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0x25, Mnemonic::And, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x35, Mnemonic::And, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x2D, Mnemonic::And, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0x3D, Mnemonic::And, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0x39, Mnemonic::And, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0x21, Mnemonic::And, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0x31, Mnemonic::And, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // ASL - Arithmetic Shift Left
    (OpCode::new(0x0A, Mnemonic::Asl, 1, 2, AddressingMode::Implicit)),
    (OpCode::new(0x06, Mnemonic::Asl, 2, 5, AddressingMode::ZeroPage)),
    (OpCode::new(0x16, Mnemonic::Asl, 2, 6, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x0E, Mnemonic::Asl, 3, 6, AddressingMode::Absolute)),
    (OpCode::new(0x1E, Mnemonic::Asl, 3, 7, AddressingMode::Absolute_X)),
    // BCC - Branch if Carry Clear
    (OpCode::new(0x90, Mnemonic::Bcc, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BCS - Branch if Carry Set
    (OpCode::new(0xB0, Mnemonic::Bcs, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BEQ - Branch if Equal
    (OpCode::new(0xF0, Mnemonic::Beq, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BIT - Bit Test
    (OpCode::new(0x24, Mnemonic::Bit, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x2C, Mnemonic::Bit, 3, 4, AddressingMode::Absolute)),
    // BMI - Branch if Minus
    (OpCode::new(0x30, Mnemonic::Bmi, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BNE - Branch if Not Equal
    (OpCode::new(0xD0, Mnemonic::Bne, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BPL - Branch if Positive
    (OpCode::new(0x10, Mnemonic::Bpl, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BRK - Force Interrupt
    (OpCode::new(0x00, Mnemonic::Brk, 1, 7, AddressingMode::Implicit)),
    // BVC - Branch if Overflow Clear
    (OpCode::new(0x50, Mnemonic::Bvc, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // BVS - Branch if Overflow Set
    (OpCode::new(0x70, Mnemonic::Bvs, 2, 2, AddressingMode::Relative)), /* +1 succeeds, +2 new page */
    // CLC - Clear Carry Flag
    (OpCode::new(0x18, Mnemonic::Clc, 1, 2, AddressingMode::Implicit)),
    // CLD - Clear Decimal Mode
    (OpCode::new(0xD8, Mnemonic::Clc, 1, 2, AddressingMode::Implicit)),
    // CLI - Clear Interrupt Disable
    (OpCode::new(0x58, Mnemonic::Clc, 1, 2, AddressingMode::Implicit)),
    // CLV - Clear Overflow Flag
    (OpCode::new(0xB8, Mnemonic::Clc, 1, 2, AddressingMode::Implicit)),
    // CMP - Compare
    (OpCode::new(0xC9, Mnemonic::Cmp, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xC5, Mnemonic::Cmp, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xD5, Mnemonic::Cmp, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0xCD, Mnemonic::Cmp, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0xDD, Mnemonic::Cmp, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0xD9, Mnemonic::Cmp, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0xC1, Mnemonic::Cmp, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0xD1, Mnemonic::Cmp, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // CPX - Compare X Register
    (OpCode::new(0xE0, Mnemonic::Cpx, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xE4, Mnemonic::Cpx, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xEC, Mnemonic::Cpx, 3, 4, AddressingMode::Absolute)),
    // CPY - Compare Y Register
    (OpCode::new(0xC0, Mnemonic::Cpy, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xC4, Mnemonic::Cpy, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xCC, Mnemonic::Cpy, 3, 4, AddressingMode::Absolute)),
    // DEC - Decrement Memory
    (OpCode::new(0xC6, Mnemonic::Dec, 2, 5, AddressingMode::ZeroPage)),
    (OpCode::new(0xD6, Mnemonic::Dec, 2, 6, AddressingMode::ZeroPage_X)),
    (OpCode::new(0xCE, Mnemonic::Dec, 3, 6, AddressingMode::Absolute)),
    (OpCode::new(0xDE, Mnemonic::Dec, 3, 7, AddressingMode::Absolute_X)),
    // DEX - Decrement X Register
    (OpCode::new(0xCA, Mnemonic::Dex, 1, 2, AddressingMode::Implicit)),
    // DEY - Decrement Y Register
    (OpCode::new(0x88, Mnemonic::Dey, 1, 2, AddressingMode::Implicit)),
    // EOR - Exclusive OR
    (OpCode::new(0x49, Mnemonic::Eor, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0x45, Mnemonic::Eor, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x55, Mnemonic::Eor, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x4D, Mnemonic::Eor, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0x5D, Mnemonic::Eor, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0x59, Mnemonic::Eor, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0x41, Mnemonic::Eor, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0x51, Mnemonic::Eor, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // INC - Increment Memory
    (OpCode::new(0xEE, Mnemonic::Inc, 2, 5, AddressingMode::ZeroPage)),
    (OpCode::new(0xF6, Mnemonic::Inc, 2, 6, AddressingMode::ZeroPage_X)),
    (OpCode::new(0xEE, Mnemonic::Inc, 3, 6, AddressingMode::Absolute)),
    (OpCode::new(0xFE, Mnemonic::Inc, 3, 7, AddressingMode::Absolute_X)),
    // INX - Increment X Register
    (OpCode::new(0xE8, Mnemonic::Inx, 1, 2, AddressingMode::Implicit)),
    // INY - Increment Y Register
    (OpCode::new(0xC8, Mnemonic::Iny, 1, 2, AddressingMode::Implicit)),
    // JMP - Jump
    (OpCode::new(0x4C, Mnemonic::Jmp, 3, 3, AddressingMode::Absolute)),
    (OpCode::new(0x6C, Mnemonic::Jmp, 3, 5, AddressingMode::Indirect)),
    // JSR - Jump to Subroutine
    (OpCode::new(0x20, Mnemonic::Jsr, 3, 6, AddressingMode::Absolute)),
    // LDA - Load Accumulator
    (OpCode::new(0xA9, Mnemonic::Lda, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xA5, Mnemonic::Lda, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xB5, Mnemonic::Lda, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0xAD, Mnemonic::Lda, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0xBD, Mnemonic::Lda, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0xB9, Mnemonic::Lda, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0xA1, Mnemonic::Lda, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0xB1, Mnemonic::Lda, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // LDX - Load X Register
    (OpCode::new(0xA2, Mnemonic::Ldx, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xA6, Mnemonic::Ldx, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xB6, Mnemonic::Ldx, 2, 4, AddressingMode::ZeroPage_Y)),
    (OpCode::new(0xAE, Mnemonic::Ldx, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0xBE, Mnemonic::Ldx, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    // LDY - Load Y Register
    (OpCode::new(0xA0, Mnemonic::Ldy, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xA4, Mnemonic::Ldy, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xB4, Mnemonic::Ldy, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0xAC, Mnemonic::Ldy, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0xBC, Mnemonic::Ldy, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    // LSR - Logical Shift Right
    (OpCode::new(0x4A, Mnemonic::Lsr, 1, 2, AddressingMode::Accumulator)),
    (OpCode::new(0x46, Mnemonic::Lsr, 2, 5, AddressingMode::ZeroPage)),
    (OpCode::new(0x56, Mnemonic::Lsr, 2, 6, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x4E, Mnemonic::Lsr, 3, 6, AddressingMode::Absolute)),
    (OpCode::new(0x5E, Mnemonic::Lsr, 3, 7, AddressingMode::Absolute_X)),
    // NOP - No Operation
    (OpCode::new(0xEA, Mnemonic::Nop, 1, 2, AddressingMode::Implicit)),
    // ORA - Logical Inclusive OR
    (OpCode::new(0x09, Mnemonic::Ora, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0x05, Mnemonic::Ora, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x15, Mnemonic::Ora, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x0D, Mnemonic::Ora, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0x1D, Mnemonic::Ora, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0x19, Mnemonic::Ora, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0x01, Mnemonic::Ora, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0x11, Mnemonic::Ora, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // PHA - Push Accumulator
    (OpCode::new(0x48, Mnemonic::Pha, 1, 3, AddressingMode::Implicit)),
    // PHP - Push Processor Status
    (OpCode::new(0x08, Mnemonic::Php, 1, 3, AddressingMode::Implicit)),
    // PLA - Pull Accumulator
    (OpCode::new(0x68, Mnemonic::Pla, 1, 4, AddressingMode::Implicit)),
    // PLP - Pull Processor Status
    (OpCode::new(0x28, Mnemonic::Plp, 1, 4, AddressingMode::Implicit)),
    // ROL - Rotate Left
    (OpCode::new(0x2A, Mnemonic::Rol, 1, 2, AddressingMode::Accumulator)),
    (OpCode::new(0x26, Mnemonic::Rol, 2, 5, AddressingMode::ZeroPage)),
    (OpCode::new(0x36, Mnemonic::Rol, 2, 6, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x2E, Mnemonic::Rol, 3, 6, AddressingMode::Absolute)),
    (OpCode::new(0x3E, Mnemonic::Rol, 3, 7, AddressingMode::Absolute_X)),
    // ROR - Rotate Right
    (OpCode::new(0x6A, Mnemonic::Ror, 1, 2, AddressingMode::Accumulator)),
    (OpCode::new(0x66, Mnemonic::Ror, 2, 5, AddressingMode::ZeroPage)),
    (OpCode::new(0x76, Mnemonic::Ror, 2, 6, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x6E, Mnemonic::Ror, 3, 6, AddressingMode::Absolute)),
    (OpCode::new(0x7E, Mnemonic::Ror, 3, 7, AddressingMode::Absolute_X)),
    // RTI - Return from Interrupt
    (OpCode::new(0x40, Mnemonic::Rti, 1, 6, AddressingMode::Implicit)),
    // RTS - Return from Subroutine
    (OpCode::new(0x60, Mnemonic::Rts, 1, 6, AddressingMode::Implicit)),
    // SBC - Subtract with Carry
    (OpCode::new(0xE9, Mnemonic::Sbc, 2, 2, AddressingMode::Immediate)),
    (OpCode::new(0xE5, Mnemonic::Sbc, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0xF5, Mnemonic::Sbc, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0xED, Mnemonic::Sbc, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0xFD, Mnemonic::Sbc, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
    (OpCode::new(0xF9, Mnemonic::Sbc, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
    (OpCode::new(0xE1, Mnemonic::Sbc, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0xF1, Mnemonic::Sbc, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
    // SEC - Set Carry Flag
    (OpCode::new(0x38, Mnemonic::Sec, 1, 2, AddressingMode::Implicit)),
    // SED - Set Decimal Flag
    (OpCode::new(0xF8, Mnemonic::Sed, 1, 2, AddressingMode::Implicit)),
    // SEI - Set Interrupt Disable
    (OpCode::new(0x78, Mnemonic::Sei, 1, 2, AddressingMode::Implicit)),
    // STA - Store Accumulator
    (OpCode::new(0x85, Mnemonic::Sta, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x95, Mnemonic::Sta, 2, 4, AddressingMode::ZeroPage_X)),
    (OpCode::new(0x8D, Mnemonic::Sta, 3, 4, AddressingMode::Absolute)),
    (OpCode::new(0x9D, Mnemonic::Sta, 3, 5, AddressingMode::Absolute_X)),
    (OpCode::new(0x99, Mnemonic::Sta, 3, 5, AddressingMode::Absolute_Y)),
    (OpCode::new(0x81, Mnemonic::Sta, 2, 6, AddressingMode::Indirect_X)),
    (OpCode::new(0x91, Mnemonic::Sta, 2, 6, AddressingMode::Indirect_Y)),
    // STX - Store X Register
    (OpCode::new(0x86, Mnemonic::Stx, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x96, Mnemonic::Stx, 2, 4, AddressingMode::ZeroPage_Y)),
    (OpCode::new(0x8E, Mnemonic::Stx, 3, 4, AddressingMode::Absolute)),
    // STY - Store Y Register
    (OpCode::new(0x84, Mnemonic::Sty, 2, 3, AddressingMode::ZeroPage)),
    (OpCode::new(0x94, Mnemonic::Sty, 2, 4, AddressingMode::ZeroPage_Y)),
    (OpCode::new(0x8C, Mnemonic::Sty, 3, 4, AddressingMode::Absolute)),
    // TAX - Transfer Accumulator to X
    (OpCode::new(0xAA, Mnemonic::Tax, 1, 2, AddressingMode::Implicit)),
    // TAY - Transfer Accumulator to Y
    (OpCode::new(0xA8, Mnemonic::Tay, 1, 2, AddressingMode::Implicit)),
    // TSX - Transfer Stack Pointer to X
    (OpCode::new(0xBA, Mnemonic::Tsx, 1, 2, AddressingMode::Implicit)),
    // TXA - Transfer X to Accumulator
    (OpCode::new(0x8A, Mnemonic::Txa, 1, 2, AddressingMode::Implicit)),
    // TXS - Transfer X to Stack Pointer
    (OpCode::new(0x9A, Mnemonic::Txs, 1, 2, AddressingMode::Implicit)),
    // TYA - Transfer Y to Accumulator
    (OpCode::new(0x98, Mnemonic::Tya, 1, 2, AddressingMode::Implicit)),
];
