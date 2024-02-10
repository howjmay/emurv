// pub mod cpu;

use crate::memory;
use crate::opcode::*;
use crate::registers;

#[derive(Debug)]
pub struct CPU {
    // integer registers
    pub xregs: registers::XREGS,
    pub pc: u64,

    pub bus: memory::BUS,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu: CPU = CPU {
            xregs: registers::XREGS::new(),
            pc: memory::MEM_BASE,
            bus: memory::BUS::new(),
        };
        cpu.xregs.regs[2] = memory::MEM_BASE + memory::MEM_SIZE; // Set stack pointer
        cpu.pc = memory::MEM_BASE;
        return cpu;
    }

    fn fetch(self) -> u32 {
        let instr: u32 = self.bus.load(self.pc, 32);
        return instr;
    }

    fn execute(&mut self, instr: u32) {
        let opcode = instr & 0x7f;
        let funct3 = (instr >> 12) & 0x7;
        let funct7 = (instr >> 25) & 0x7f;
        self.xregs.regs[0] = 0; // x0 hardwired to 0 at each cycle

        match instr {
            LUI => exec_lui(self, instr),
            AUIPC => exec_auipc(self, instr),
            JAL => exec_jal(self, instr),
            JALR => exec_jalr(self, instr),
            B_TYPE => match funct7 {
                BEQ => exec_beq(self, instr),
                BNE => exec_bne(self, instr),
                BLT => exec_blt(self, instr),
                BGE => exec_bge(self, instr),
                BLTU => exec_bltu(self, instr),
                BGEU => exec_bgeu(self, instr),
                _ => panic!(),
            },
            LOAD => match funct3 {
                LB => exec_lb(self, instr),
                LH => exec_lh(self, instr),
                LW => exec_lw(self, instr),
                LD => exec_ld(self, instr),
                LBU => exec_lbu(self, instr),
                LHU => exec_lhu(self, instr),
                LWU => exec_lwu(self, instr),
                _ => panic!(),
            },
            S_TYPE => match funct3 {
                SB => exec_sb(self, instr),
                SH => exec_sh(self, instr),
                SW => exec_sw(self, instr),
                SD => exec_sd(self, instr),
                _ => panic!(),
            },
            I_TYPE => match funct3 {
                ADDI => exec_addi(self, instr),
                SLLI => exec_slli(self, instr),
                SLTI => exec_slti(self, instr),
                SLTIU => exec_sltiu(self, instr),
                XORI => exec_xori(self, instr),
                SRI => match funct7 {
                    SRLI => exec_srli(self, instr),
                    SRAI => exec_srai(self, instr),
                    _ => panic!(),
                },
                ORI => exec_ori(self, instr),
                ANDI => exec_andi(self, instr),
                _ => {
                    panic!("malformed I type instruction");
                }
            },
            R_TYPE => match funct3 {
                ADDSUB => match funct7 {
                    ADD => exec_add(self, instr),
                    SUB => exec_sub(self, instr),
                    _ => (),
                },
                SLL => exec_sll(self, instr),
                SLT => exec_slt(self, instr),
                SLTU => exec_sltu(self, instr),
                XOR => exec_xor(self, instr),
                SR => match funct7 {
                    SRL => exec_srl(self, instr),
                    SRA => exec_sra(self, instr),
                    _ => (),
                },
                OR => exec_or(self, instr),
                AND => exec_and(self, instr),
                _ => {
                    panic!("malformed I type instruction");
                }
            },
            FENCE => exec_fence(self, instr),
            _ => panic!(),
        }
    }
}

// RV32I
// see page 64 at https://riscv.org/wp-content/uploads/2016/06/riscv-spec-v2.1.pdf
pub fn exec_lui(cpu: &mut CPU, instr: u32) {
    let imm = (imm_U(instr) as i32) as u64;
    cpu.xregs.regs[rd(instr) as usize] = imm;
}
pub fn exec_auipc(cpu: &mut CPU, instr: u32) {
    let imm = (imm_U(instr) as i32) as i64;
    cpu.xregs.regs[rd(instr) as usize] = (cpu.pc as i64 + imm) as u64;
}
pub fn exec_jal(cpu: &mut CPU, instr: u32) {
    let imm = (imm_J(instr) as i32) as i64;
    cpu.xregs.regs[rd(instr) as usize] = cpu.pc + 4;
    cpu.pc = (cpu.pc as i64 + imm) as u64;
}
pub fn exec_jalr(cpu: &mut CPU, instr: u32) {
    let imm = (imm_J(instr) as i32) as i64;
    cpu.xregs.regs[rd(instr) as usize] = cpu.pc + 4;
    cpu.pc = (cpu.pc as i64 + imm) as u64;
}
pub fn exec_beq(cpu: &mut CPU, instr: u32) {}
pub fn exec_bne(cpu: &mut CPU, instr: u32) {}
pub fn exec_blt(cpu: &mut CPU, instr: u32) {}
pub fn exec_bge(cpu: &mut CPU, instr: u32) {}
pub fn exec_bltu(cpu: &mut CPU, instr: u32) {}
pub fn exec_bgeu(cpu: &mut CPU, instr: u32) {}
pub fn exec_lb(cpu: &mut CPU, instr: u32) {}
pub fn exec_lh(cpu: &mut CPU, instr: u32) {}
pub fn exec_lw(cpu: &mut CPU, instr: u32) {}
pub fn exec_ld(cpu: &mut CPU, instr: u32) {}
pub fn exec_lbu(cpu: &mut CPU, instr: u32) {}
pub fn exec_lhu(cpu: &mut CPU, instr: u32) {}
pub fn exec_lwu(cpu: &mut CPU, instr: u32) {}
pub fn exec_sb(cpu: &mut CPU, instr: u32) {}
pub fn exec_sh(cpu: &mut CPU, instr: u32) {}
pub fn exec_sw(cpu: &mut CPU, instr: u32) {}
pub fn exec_sd(cpu: &mut CPU, instr: u32) {}
pub fn exec_addi(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] + imm as u64;
}
pub fn exec_slti(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] =
        ((cpu.xregs.regs[rs1(instr) as usize] as i64) < (imm as i64)) as u64;
}
pub fn exec_sltiu(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = (cpu.xregs.regs[rs1(instr) as usize] < imm as u64) as u64;
}
pub fn exec_xori(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] ^ imm as u64;
}
pub fn exec_ori(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] | imm as u64;
}
pub fn exec_andi(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] & imm as u64;
}
pub fn exec_slli(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] << imm as u64;
}
pub fn exec_srli(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] >> imm as u64;
}
pub fn exec_srai(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = (cpu.xregs.regs[rs1(instr) as usize] as i64 >> imm) as u64;
}
pub fn exec_add(cpu: &mut CPU, instr: u32) {
    cpu.xregs.regs[rd(instr) as usize] = (cpu.xregs.regs[rs1(instr) as usize] as i64
        + cpu.xregs.regs[rs2(instr) as usize] as i64)
        as u64;
}
pub fn exec_sub(cpu: &mut CPU, instr: u32) {
    cpu.xregs.regs[rd(instr) as usize] = (cpu.xregs.regs[rs1(instr) as usize] as i64
        - cpu.xregs.regs[rs2(instr) as usize] as i64)
        as u64;
}
pub fn exec_sll(cpu: &mut CPU, instr: u32) {}
pub fn exec_slt(cpu: &mut CPU, instr: u32) {}
pub fn exec_sltu(cpu: &mut CPU, instr: u32) {}
pub fn exec_xor(cpu: &mut CPU, instr: u32) {}
pub fn exec_srl(cpu: &mut CPU, instr: u32) {}
pub fn exec_sra(cpu: &mut CPU, instr: u32) {}
pub fn exec_or(cpu: &mut CPU, instr: u32) {}
pub fn exec_and(cpu: &mut CPU, instr: u32) {}
pub fn exec_fence(cpu: &mut CPU, instr: u32) {}
pub fn exec_fence_i(cpu: &mut CPU, instr: u32) {}
pub fn exec_ecall(cpu: &mut CPU, instr: u32) {}
pub fn exec_ebreak(cpu: &mut CPU, instr: u32) {}
pub fn exec_csrrw(cpu: &mut CPU, instr: u32) {}
pub fn exec_csrrs(cpu: &mut CPU, instr: u32) {}
pub fn exec_csrrc(cpu: &mut CPU, instr: u32) {}
pub fn exec_csrrwi(cpu: &mut CPU, instr: u32) {}
pub fn exec_csrrsi(cpu: &mut CPU, instr: u32) {}
pub fn exec_csrrci(cpu: &mut CPU, instr: u32) {}
