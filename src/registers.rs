use core::fmt;

#[derive(Clone, Copy)]
pub struct XREGS {
    pub regs: [u32; 32],
}

impl XREGS {
    pub fn new() -> Self {
        XREGS { regs: [0; 32] }
    }
}

impl fmt::Debug for XREGS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("xregs");
        for i in 0..32 {
            s.field(format!("xreg[{i}]").as_str(), &self.regs[i]);
        }
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xregs_debug() {
        let mut xregs = XREGS::new();
        for i in 0..32 {
            xregs.regs[i] = (i * 11) as u32;
        }
        println!("{xregs:#?}")
    }
}
