use crate::cpu;

pub const REGS_NAMES: &[&str] = &[
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub fn dump_registers(cpu: &cpu::CPU) {
    for i in 0..8 {
        print!("{:4}: {:#13x}  ", REGS_NAMES[i], cpu.xregs.regs[i]);
        print!("{:4}: {:#13x}  ", REGS_NAMES[i + 8], cpu.xregs.regs[i + 8]);
        print!(
            "{:4}: {:#13x}  ",
            REGS_NAMES[i + 16],
            cpu.xregs.regs[i + 16]
        );
        print!(
            "{:4}: {:#13x}\n",
            REGS_NAMES[i + 24],
            cpu.xregs.regs[i + 24]
        );
    }
}
