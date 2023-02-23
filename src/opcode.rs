use crate::cpu::AddressingMode;
use once_cell::unsync::Lazy;
use std::{borrow::Borrow, collections::HashSet};

type Code = u8;

#[derive(Debug, Eq)]
pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
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
    pub fn new(
        code: u8,
        mnemonic: &'static str,
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

    #[rustfmt::skip]
    pub fn get_instruction_codes() -> Lazy<HashSet<OpCode>> {
        Lazy::new(|| {
            HashSet::from([
                (Self::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing)),
                (Self::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing)),
                (Self::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing)),
                
                /* LDA */
                (Self::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate)),
                (Self::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage)),
                (Self::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X)),
                (Self::new(0xad, "LDA", 3, 4, AddressingMode::Absolute)),
                (Self::new(0xbd, "LDA", 3, 4/* +1 if page crossed */, AddressingMode::Absolute_X)),
                (Self::new(0xb9, "LDA", 3, 4/* +1 if page crossed */, AddressingMode::Absolute_Y)),
                (Self::new(0xa1, "LDA", 2, 6                        , AddressingMode::Indirect_X)),
                (Self::new(0xb1, "LDA", 2, 5/* +1 if page crossed */, AddressingMode::Indirect_Y)),
                
                /* STA */
                (Self::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage)),
                (Self::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X)),
                (Self::new(0x8d, "STA", 3, 4, AddressingMode::Absolute)),
                (Self::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X)),
                (Self::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y)),
                (Self::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X)),
                (Self::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y)),

                /* ADC */
                (Self::new(0x69, "ADC", 2, 2, AddressingMode::Immediate)),
                (Self::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage)),
                (Self::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X)),
                (Self::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute)),
                (Self::new(0x7d, "ADC", 3, 4/* +1 if page crossed */, AddressingMode::Absolute_X)),
                (Self::new(0x79, "ADC", 3, 4/* +1 if page crossed */, AddressingMode::Absolute_Y)),
                (Self::new(0x61, "ADC", 2, 6                        , AddressingMode::Indirect_X)),
                (Self::new(0x71, "ADC", 2, 5/* +1 if page crossed */, AddressingMode::Indirect_Y)),
                
                /* AND */
                (Self::new(0x29, "AND", 2, 2, AddressingMode::Immediate)),
                (Self::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage)),
                (Self::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X)),
                (Self::new(0x2d, "AND", 3, 4, AddressingMode::Absolute)),
                (Self::new(0x3d, "AND", 3, 4/* +1 if page crossed */, AddressingMode::Absolute_X)),
                (Self::new(0x39, "AND", 3, 4/* +1 if page crossed */, AddressingMode::Absolute_Y)),
                (Self::new(0x21, "AND", 2, 6                        , AddressingMode::Indirect_X)),
                (Self::new(0x31, "AND", 2, 5/* +1 if page crossed */, AddressingMode::Indirect_Y)),
                
                /* ASL - Arithmetic Shift Left */
                (Self::new(0x0a, "ASL", 1, 2, AddressingMode::NoneAddressing)),
                (Self::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage)),
                (Self::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X)),
                (Self::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute)),
                (Self::new(0x1e, "ASL", 3, 7, AddressingMode::Absolute_X)),

            ])
        })
    }
}
