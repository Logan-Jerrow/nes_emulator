use crate::cpu::AddressingMode;
use once_cell::unsync::Lazy;
use std::{borrow::Borrow, collections::HashSet, str::FromStr};

type Code = u8;

#[derive(Debug, Eq)] // PartialEq see impl
pub struct OpCode {
    pub code: u8,
    pub mnemonic: Mnemonic,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mnemonic {
    Adc,
    And,
    Asl,
    Lda,
    Sta,
    Brk,
    Tax,
    Inx,
}

impl PartialEq for OpCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl std::hash::Hash for OpCode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl Borrow<Code> for OpCode {
    fn borrow(&self) -> &Code {
        &self.code
    }
}

impl OpCode {
    pub const fn new(
        code: u8,
        mnemonic: Mnemonic,
        len: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }

    pub const INSTRUCTIONS: [Self; 39] = [
        (Self::new(0x00, Mnemonic::Brk, 1, 7, AddressingMode::NoneAddressing)),
        (Self::new(0xaa, Mnemonic::Tax, 1, 2, AddressingMode::NoneAddressing)),
        (Self::new(0xe8, Mnemonic::Inx, 1, 2, AddressingMode::NoneAddressing)),
        /* LDA */
        (Self::new(0xa9, Mnemonic::Lda, 2, 2, AddressingMode::Immediate)),
        (Self::new(0xa5, Mnemonic::Lda, 2, 3, AddressingMode::ZeroPage)),
        (Self::new(0xb5, Mnemonic::Lda, 2, 4, AddressingMode::ZeroPage_X)),
        (Self::new(0xad, Mnemonic::Lda, 3, 4, AddressingMode::Absolute)),
        (Self::new(0xbd, Mnemonic::Lda, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
        (Self::new(0xb9, Mnemonic::Lda, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
        (Self::new(0xa1, Mnemonic::Lda, 2, 6, AddressingMode::Indirect_X)),
        (Self::new(0xb1, Mnemonic::Lda, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
        /* STA */
        (Self::new(0x85, Mnemonic::Sta, 2, 3, AddressingMode::ZeroPage)),
        (Self::new(0x95, Mnemonic::Sta, 2, 4, AddressingMode::ZeroPage_X)),
        (Self::new(0x8d, Mnemonic::Sta, 3, 4, AddressingMode::Absolute)),
        (Self::new(0x9d, Mnemonic::Sta, 3, 5, AddressingMode::Absolute_X)),
        (Self::new(0x99, Mnemonic::Sta, 3, 5, AddressingMode::Absolute_Y)),
        (Self::new(0x81, Mnemonic::Sta, 2, 6, AddressingMode::Indirect_X)),
        (Self::new(0x91, Mnemonic::Sta, 2, 6, AddressingMode::Indirect_Y)),
        /* ADC */
        (Self::new(0x69, Mnemonic::Adc, 2, 2, AddressingMode::Immediate)),
        (Self::new(0x65, Mnemonic::Adc, 2, 3, AddressingMode::ZeroPage)),
        (Self::new(0x75, Mnemonic::Adc, 2, 4, AddressingMode::ZeroPage_X)),
        (Self::new(0x6d, Mnemonic::Adc, 3, 4, AddressingMode::Absolute)),
        (Self::new(0x7d, Mnemonic::Adc, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
        (Self::new(0x79, Mnemonic::Adc, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
        (Self::new(0x61, Mnemonic::Adc, 2, 6, AddressingMode::Indirect_X)),
        (Self::new(0x71, Mnemonic::Adc, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
        /* AND */
        (Self::new(0x29, Mnemonic::And, 2, 2, AddressingMode::Immediate)),
        (Self::new(0x25, Mnemonic::And, 2, 3, AddressingMode::ZeroPage)),
        (Self::new(0x35, Mnemonic::And, 2, 4, AddressingMode::ZeroPage_X)),
        (Self::new(0x2d, Mnemonic::And, 3, 4, AddressingMode::Absolute)),
        (Self::new(0x3d, Mnemonic::And, 3, 4, AddressingMode::Absolute_X)), /* +1 if page crossed */
        (Self::new(0x39, Mnemonic::And, 3, 4, AddressingMode::Absolute_Y)), /* +1 if page crossed */
        (Self::new(0x21, Mnemonic::And, 2, 6, AddressingMode::Indirect_X)),
        (Self::new(0x31, Mnemonic::And, 2, 5, AddressingMode::Indirect_Y)), /* +1 if page crossed */
        /* ASL - Arithmetic Shift Left */
        (Self::new(0x0a, Mnemonic::Asl, 1, 2, AddressingMode::NoneAddressing)),
        (Self::new(0x06, Mnemonic::Asl, 2, 5, AddressingMode::ZeroPage)),
        (Self::new(0x16, Mnemonic::Asl, 2, 6, AddressingMode::ZeroPage_X)),
        (Self::new(0x0e, Mnemonic::Asl, 3, 6, AddressingMode::Absolute)),
        (Self::new(0x1e, Mnemonic::Asl, 3, 7, AddressingMode::Absolute_X)),
    ];

    pub fn get_instruction_codes() -> Lazy<HashSet<Self>> {
        Lazy::new(|| HashSet::from(Self::INSTRUCTIONS))
    }
}
