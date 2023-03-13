use bitflags::bitflags;

impl Default for CpuFlags {
    fn default() -> Self {
        Self::INIT
    }
}

bitflags! {
    /// # Status Register (P)
    ///
    /// http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    /// Processor status (P) - 8-bit register represents 7 status flags that can be set or unset
    /// depending on the result of the last executed instruction (for example Z flag is set (1) if the
    /// result of an operation is 0, and is unset/erased (0) otherwise)
    ///
    /// # Figure
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    pub struct CpuFlags: u8{
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERUPT_DISABLE  = 0b0000_0100;
        const DECIMAL_MODE      = 0b0000_1000;
        const BREAK             = 0b0001_0000;
        const BREAK2            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIV           = 0b1000_0000;

        const INIT = Self::BREAK2.bits | Self::CARRY.bits;
    }
}
