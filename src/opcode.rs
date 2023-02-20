use crate::cpu::AddressingMode;
use once_cell::unsync::Lazy;
use std::collections::HashSet;

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

    pub fn get_codes() -> Lazy<HashSet<OpCode>> {
        Lazy::new(|| {
            let mut set = HashSet::new();
            set.insert(Self::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing));
            set.insert(Self::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing));
            set.insert(Self::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing));

            /* LDA */
            set.insert(Self::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate));
            set.insert(Self::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage));
            set.insert(Self::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X));
            set.insert(Self::new(0xad, "LDA", 3, 4, AddressingMode::Absolute));
            set.insert(Self::new(0xbd, "LDA", 3, 4, AddressingMode::Absolute_X)); /* +1 if page crossed */
            set.insert(Self::new(0xb9, "LDA", 3, 4, AddressingMode::Absolute_Y)); /* +1 if page crossed */
            set.insert(Self::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X));
            set.insert(Self::new(0xb1, "LDA", 2, 5, AddressingMode::Indirect_Y)); /* +1 if page crossed */

            /* STA */
            set.insert(Self::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage));
            set.insert(Self::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X));
            set.insert(Self::new(0x8d, "STA", 3, 4, AddressingMode::Absolute));
            set.insert(Self::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X));
            set.insert(Self::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y));
            set.insert(Self::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X));
            set.insert(Self::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y));

            set
        })
    }
}
