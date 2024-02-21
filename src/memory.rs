// pub mod memory;

pub const MEM_BASE: u32 = 0x80000000; // defined in QEMU
pub const MEM_SIZE: u32 = 1024;

#[derive(Debug, Clone, Copy)]
pub struct BUS {
    mem: MEMORY,
}

impl BUS {
    pub fn new() -> Self {
        BUS { mem: MEMORY::new() }
    }
    pub fn load(self, addr: u32, size: u32) -> u32 {
        return self.mem.load(addr, size) as u32;
    }
    pub fn store(&mut self, addr: u32, size: u32, value: u32) {
        self.mem.store(addr, size, value);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MEMORY {
    mem: [u8; MEM_SIZE as usize],
}

impl MEMORY {
    fn new() -> Self {
        MEMORY {
            mem: [0; MEM_SIZE as usize],
        }
    }

    fn load(self, addr: u32, size: u32) -> u32 {
        match size {
            8 => return self.load8(addr),
            16 => return self.load16(addr),
            32 => return self.load32(addr),
            // 64 => return self.load64(addr),
            _ => panic!("wrong load size"),
        }
    }
    fn store(&mut self, addr: u32, size: u32, value: u32) {
        match size {
            8 => self.store8(addr, value),
            16 => self.store16(addr, value),
            32 => self.store32(addr, value),
            // 64 => self.store64(addr, value),
            _ => panic!("wrong store size"),
        }
    }

    // load funcs
    fn load8(self, addr: u32) -> u32 {
        let index = (addr - MEM_BASE) as usize;
        return self.mem[index] as u32;
    }
    fn load16(self, addr: u32) -> u32 {
        let index = (addr - MEM_BASE) as usize;
        return self.mem[index] as u32 | ((self.mem[index + 1] as u32) << 8);
    }
    fn load32(self, addr: u32) -> u32 {
        let index = (addr - MEM_BASE) as usize;
        return self.mem[index] as u32
            | ((self.mem[index + 1] as u32) << 8)
            | ((self.mem[index + 2] as u32) << 16)
            | ((self.mem[index + 3] as u32) << 24);
    }
    // fn load64(self, addr: u32) -> u32 {
    //     let index = (addr - MEM_BASE) as usize;
    //     return self.mem[index] as u32
    //         | ((self.mem[index + 1] as u32) << 8)
    //         | ((self.mem[index + 2] as u32) << 16)
    //         | ((self.mem[index + 3] as u32) << 24)
    //         | ((self.mem[index + 4] as u32) << 32)
    //         | ((self.mem[index + 5] as u32) << 40)
    //         | ((self.mem[index + 6] as u32) << 48)
    //         | ((self.mem[index + 7] as u32) << 56);
    // }

    // store funcs
    fn store8(&mut self, addr: u32, value: u32) {
        let index = (addr - MEM_BASE) as usize;
        self.mem[index] = (value & (std::u8::MAX as u32)) as u8;
    }
    fn store16(&mut self, addr: u32, value: u32) {
        let index = (addr - MEM_BASE) as usize;
        self.mem[index] = (value & (std::u8::MAX as u32)) as u8;
        self.mem[index + 1] = ((value >> 8) & (std::u8::MAX as u32)) as u8;
    }
    fn store32(&mut self, addr: u32, value: u32) {
        let index = (addr - MEM_BASE) as usize;
        self.mem[index] = (value & (std::u8::MAX as u32)) as u8;
        self.mem[index + 1] = ((value >> 8) & (std::u8::MAX as u32)) as u8;
        self.mem[index + 2] = ((value >> 16) & (std::u8::MAX as u32)) as u8;
        self.mem[index + 3] = ((value >> 24) & (std::u8::MAX as u32)) as u8;
    }
    // fn store64(&mut self, addr: u32, value: u32) {
    //     let index = (addr - MEM_BASE) as usize;
    //     self.mem[index] = (value & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 1] = ((value >> 8) & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 2] = ((value >> 16) & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 3] = ((value >> 24) & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 4] = ((value >> 32) & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 5] = ((value >> 40) & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 6] = ((value >> 48) & (std::u8::MAX as u32)) as u8;
    //     self.mem[index + 7] = ((value >> 56) & (std::u8::MAX as u32)) as u8;
    // }
}
