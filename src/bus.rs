use crate::cpu::memory::Memory;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const VRAM: usize = 2048; // 2^11
pub struct Bus {
    cpu_vram: [u8; VRAM],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            cpu_vram: [0; VRAM],
        }
    }
}

impl Memory for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        const RAM_ADDR_BITS: u16 = 0b0000_0111_1111_1111;
        const PPU_ADDR_BITS: u16 = 0b0010_0000_0000_0111;

        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & RAM_ADDR_BITS;
                self.cpu_vram[mirror_down_addr as usize]
            }

            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & PPU_ADDR_BITS;
                todo!()
            }

            _ => {
                println!("Ignoring invalid memory access at {addr:#04x}");
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & ELEVEN_BITS;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }

            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & PPU_ADDR_BITS;
                todo!();
            }

            _ => {
                println!("Ignoring invalid memory access at {addr:#04x}");
            }
        }
    }
}
