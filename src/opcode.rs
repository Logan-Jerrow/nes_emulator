use self::mnemonic::Mnemonic;
use crate::addressing_mode::AddressingMode;
use std::{borrow::Borrow, collections::HashSet, str::FromStr};

pub mod mnemonic;

pub type Raw = u8;

#[derive(Debug, Clone, Copy, Eq)] // PartialEq see impl
pub struct OpCode {
    pub code: Raw,
    pub mnemonic: Mnemonic,
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

impl Borrow<Raw> for OpCode {
    fn borrow(&self) -> &Raw {
        &self.code
    }
}

impl OpCode {
    pub const fn new(
        code: Raw,
        mnemonic: Mnemonic,
        len: u8,
        cycles: u8,
        addr: AddressingMode,
    ) -> Self {
        Self {
            code,
            mnemonic,
            len,
            cycles,
            mode: addr,
        }
    }
}
