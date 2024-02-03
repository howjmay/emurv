// pub mod cpu;

use crate::memory;
use crate::opcode::*;
use crate::registers;

#[derive(Debug)]
pub struct CPU {
    // integer registers
    pub xregs: registers::XREGS,
    pc: u64,

    bus: memory::BUS,
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
            LUI => (),
            AUIPC => (),
            JAL => (),
            JALR => (),
            B_TYPE => match funct7 {
                BEQ => exec_beq(self, instr),
                BNE => exec_bne(self, instr),
                BLT => exec_blt(self, instr),
                BGE => exec_bge(self, instr),
                BLTU => exec_bltu(self, instr),
                BGEU => exec_bgeu(self, instr),
                _ => panic!(),
            },
            LOAD => match (funct3) {
                LB => exec_lb(self, instr),
                LH => exec_lh(self, instr),
                LW => exec_lw(self, instr),
                LD => exec_ld(self, instr),
                LBU => exec_lbu(self, instr),
                LHU => exec_lhu(self, instr),
                LWU => exec_lwu(self, instr),
                _ => panic!(),
            },
            S_TYPE => match (funct3) {
                SB => exec_sb(self, instr),
                SH => exec_sh(self, instr),
                SW => exec_sw(self, instr),
                SD => exec_sd(self, instr),
                _ => panic!(),
            },
            I_TYPE => match (funct3) {
                ADDI => exec_addi(self, instr),
                SLLI => exec_slli(self, instr),
                SLTI => exec_slti(self, instr),
                SLTIU => exec_sltiu(self, instr),
                XORI => exec_xori(self, instr),
                SRI => match (funct7) {
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
            R_TYPE => match (funct3) {
                ADDSUB => match (funct7) {
                    ADD => exec_add(self, instr),
                    SUB => exec_sub(self, instr),
                    _ => (),
                },
                SLL => exec_sll(self, instr),
                SLT => exec_slt(self, instr),
                SLTU => exec_sltu(self, instr),
                XOR => exec_xor(self, instr),
                SR => match (funct7) {
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

fn rd(instr: u32) -> u32 {
    return (instr >> 7) & 0x1f; // rd in bits 11..7
}
fn rs1(instr: u32) -> u32 {
    return (instr >> 15) & 0x1f; // rs1 in bits 19..15
}
fn rs2(instr: u32) -> u32 {
    return (instr >> 20) & 0x1f; // rs2 in bits 24..20
}

fn imm_I(instr: u32) -> u32 {
    return (instr & 0xfff00000) >> 20;
}

// uint64_t imm_S(uint32_t inst) {
//     // imm[11:5] = inst[31:25], imm[4:0] = inst[11:7]
//     return ((int64_t)(int32_t)(inst & 0xfe000000) >> 20)
//         | ((inst >> 7) & 0x1f);
// }
// uint64_t imm_B(uint32_t inst) {
//     // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
//     return ((int64_t)(int32_t)(inst & 0x80000000) >> 19)
//         | ((inst & 0x80) << 4) // imm[11]
//         | ((inst >> 20) & 0x7e0) // imm[10:5]
//         | ((inst >> 7) & 0x1e); // imm[4:1]
// }
// uint64_t imm_U(uint32_t inst) {
//     // imm[31:12] = inst[31:12]
//     return (int64_t)(int32_t)(inst & 0xfffff999);
// }
// uint64_t imm_J(uint32_t inst) {
//     // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
//     return (uint64_t)((int64_t)(int32_t)(inst & 0x80000000) >> 11)
//         | (inst & 0xff000) // imm[19:12]
//         | ((inst >> 9) & 0x800) // imm[11]
//         | ((inst >> 20) & 0x7fe); // imm[10:1]
// }

// uint32_t shamt(uint32_t inst) {
//     // shamt(shift amount) only required for immediate shift instructions
//     // shamt[4:5] = imm[5:0]
//     return (uint32_t) (imm_I(inst) & 0x1f); // TODO: 0x1f / 0x3f ?
// }

// uint64_t csr(uint32_t inst) {
//     // csr[11:0] = inst[31:20]
//     return ((inst & 0xfff00000) >> 20);
// }

// RV32I
// see page 64 at https://riscv.org/wp-content/uploads/2016/06/riscv-spec-v2.1.pdf
pub fn exec_lui(cpu: &CPU, instr: u32) {}
pub fn exec_auipc(cpu: &CPU, instr: u32) {}
pub fn exec_jal(cpu: &CPU, instr: u32) {}
pub fn exec_jalr(cpu: &CPU, instr: u32) {}
pub fn exec_beq(cpu: &CPU, instr: u32) {}
pub fn exec_bne(cpu: &CPU, instr: u32) {}
pub fn exec_blt(cpu: &CPU, instr: u32) {}
pub fn exec_bge(cpu: &CPU, instr: u32) {}
pub fn exec_bltu(cpu: &CPU, instr: u32) {}
pub fn exec_bgeu(cpu: &CPU, instr: u32) {}
pub fn exec_lb(cpu: &CPU, instr: u32) {}
pub fn exec_lh(cpu: &CPU, instr: u32) {}
pub fn exec_lw(cpu: &CPU, instr: u32) {}
pub fn exec_ld(cpu: &CPU, instr: u32) {}
pub fn exec_lbu(cpu: &CPU, instr: u32) {}
pub fn exec_lhu(cpu: &CPU, instr: u32) {}
pub fn exec_lwu(cpu: &CPU, instr: u32) {}
pub fn exec_sb(cpu: &CPU, instr: u32) {}
pub fn exec_sh(cpu: &CPU, instr: u32) {}
pub fn exec_sw(cpu: &CPU, instr: u32) {}
pub fn exec_sd(cpu: &CPU, instr: u32) {}
pub fn exec_addi(cpu: &mut CPU, instr: u32) {
    let imm = imm_I(instr);
    cpu.xregs.regs[rd(instr) as usize] = cpu.xregs.regs[rs1(instr) as usize] + imm as u64;
}
pub fn exec_slti(cpu: &CPU, instr: u32) {}
pub fn exec_sltiu(cpu: &CPU, instr: u32) {}
pub fn exec_xori(cpu: &CPU, instr: u32) {}
pub fn exec_ori(cpu: &CPU, instr: u32) {}
pub fn exec_andi(cpu: &CPU, instr: u32) {}
pub fn exec_slli(cpu: &CPU, instr: u32) {}
pub fn exec_srli(cpu: &CPU, instr: u32) {}
pub fn exec_srai(cpu: &CPU, instr: u32) {}
pub fn exec_add(cpu: &CPU, instr: u32) {}
pub fn exec_sub(cpu: &CPU, instr: u32) {}
pub fn exec_sll(cpu: &CPU, instr: u32) {}
pub fn exec_slt(cpu: &CPU, instr: u32) {}
pub fn exec_sltu(cpu: &CPU, instr: u32) {}
pub fn exec_xor(cpu: &CPU, instr: u32) {}
pub fn exec_srl(cpu: &CPU, instr: u32) {}
pub fn exec_sra(cpu: &CPU, instr: u32) {}
pub fn exec_or(cpu: &CPU, instr: u32) {}
pub fn exec_and(cpu: &CPU, instr: u32) {}
pub fn exec_fence(cpu: &CPU, instr: u32) {}
pub fn exec_fence_i(cpu: &CPU, instr: u32) {}
pub fn exec_ecall(cpu: &CPU, instr: u32) {}
pub fn exec_ebreak(cpu: &CPU, instr: u32) {}
pub fn exec_csrrw(cpu: &CPU, instr: u32) {}
pub fn exec_csrrs(cpu: &CPU, instr: u32) {}
pub fn exec_csrrc(cpu: &CPU, instr: u32) {}
pub fn exec_csrrwi(cpu: &CPU, instr: u32) {}
pub fn exec_csrrsi(cpu: &CPU, instr: u32) {}
pub fn exec_csrrci(cpu: &CPU, instr: u32) {}
