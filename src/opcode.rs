pub const LUI: u32 = 0x37;
pub const AUIPC: u32 = 0x17;

pub const JAL: u32 = 0x6f;
pub const JALR: u32 = 0x67;

pub const B_TYPE: u32 = 0x63;
pub const BEQ: u32 = 0x0;
pub const BNE: u32 = 0x1;
pub const BLT: u32 = 0x4;
pub const BGE: u32 = 0x5;
pub const BLTU: u32 = 0x6;
pub const BGEU: u32 = 0x7;

pub const LOAD: u32 = 0x03;
pub const LB: u32 = 0x0;
pub const LH: u32 = 0x1;
pub const LW: u32 = 0x2;
pub const LD: u32 = 0x3;
pub const LBU: u32 = 0x4;
pub const LHU: u32 = 0x5;
pub const LWU: u32 = 0x6;

pub const S_TYPE: u32 = 0x23;
pub const SB: u32 = 0x0;
pub const SH: u32 = 0x1;
pub const SW: u32 = 0x2;
pub const SD: u32 = 0x3;

pub const I_TYPE: u32 = 0x13;
pub const ADDI: u32 = 0x0;
pub const SLLI: u32 = 0x1;
pub const SLTI: u32 = 0x2;
pub const SLTIU: u32 = 0x3;
pub const XORI: u32 = 0x4;
pub const SRI: u32 = 0x5;
pub const SRLI: u32 = 0x00;
pub const SRAI: u32 = 0x20;
pub const ORI: u32 = 0x6;
pub const ANDI: u32 = 0x7;

pub const R_TYPE: u32 = 0x33;
pub const ADDSUB: u32 = 0x0;
pub const ADD: u32 = 0x00;
pub const SUB: u32 = 0x20;
pub const SLL: u32 = 0x1;
pub const SLT: u32 = 0x2;
pub const SLTU: u32 = 0x3;
pub const XOR: u32 = 0x4;
pub const SR: u32 = 0x5;
pub const SRL: u32 = 0x00;
pub const SRA: u32 = 0x20;
pub const OR: u32 = 0x6;
pub const AND: u32 = 0x7;

pub const FENCE: u32 = 0x0f;

// pub const I_TYPE_64: u32 = 0x1b;
// pub const ADDIW: u32 = 0x0;
// pub const SLLIW: u32 = 0x1;
// pub const SRIW: u32 = 0x5;
// pub const SRLIW: u32 = 0x00;
// pub const SRAIW: u32 = 0x20;

// pub const R_TYPE_64: u32 = 0x3b;
// pub const ADDSUB: u32 = 0x0;
// pub const ADDW: u32 = 0x00;
// pub const MULW: u32 = 0x01;
// pub const SUBW: u32 = 0x20;
// pub const DIVW: u32 = 0x4;
// pub const SLLW: u32 = 0x1;
// pub const SRW: u32 = 0x5;
// pub const SRLW: u32 = 0x00;
// pub const DIVUW: u32 = 0x01;
// pub const SRAW: u32 = 0x20;
// pub const REMW: u32 = 0x6;
// pub const REMUW: u32 = 0x7;

pub const CSR: u32 = 0x73;
pub const ECALL: u32 = 0x00;
pub const EBREAK: u32 = 0x00;
pub const CSRRW: u32 = 0x01;
pub const CSRRS: u32 = 0x02;
pub const CSRRC: u32 = 0x03;
pub const CSRRWI: u32 = 0x05;
pub const CSRRSI: u32 = 0x06;
pub const CSRRCI: u32 = 0x07;

pub fn rd(instr: u32) -> u32 {
    return (instr >> 7) & 0x1f; // rd in bits 11..7
}
pub fn rs1(instr: u32) -> u32 {
    return (instr >> 15) & 0x1f; // rs1 in bits 19..15
}
pub fn rs2(instr: u32) -> u32 {
    return (instr >> 20) & 0x1f; // rs2 in bits 24..20
}

pub fn shamt(instr: u32) -> u32 {
    // shamt[4:5] = imm[5:0]
    return (imm_i(instr) & 0x1f) as u32;
}

pub fn csr(instr: u32) -> u32 {
    // csr[11:0] = inst[31:20]
    return ((instr & 0xfff00000) >> 20) as u32;
}

pub fn imm_b(instr: u32) -> u32 {
    // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
    return ((instr & 0x80000000) as i32 >> 19) as u32
        | ((instr & 0x80) << 4) as u32 // imm[11]
        | ((instr >> 20) & 0x7e0) as u32 // imm[10:5]
        | ((instr >> 7) & 0x1e) as u32; // imm[4:1]
}

pub fn imm_s(instr: u32) -> u32 {
    // imm[11:5] = inst[31:25], imm[4:0] = inst[11:7]
    return ((instr & 0xfe000000) >> 20) as u32 | ((instr >> 7) & 0x1f) as u32;
}

pub fn imm_i(instr: u32) -> i32 {
    // imm[11:0] = inst[31:20]
    return (instr & 0xfff00000) as i32 >> 20;
}

pub fn imm_u(instr: u32) -> u32 {
    // imm[31:12] = inst[31:12]
    return (instr & 0xfffff000) as u32;
}

pub fn imm_j(instr: u32) -> u32 {
    // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
    return (((instr & 0x80000000) as i32 >> 11) as u32)// imm[20]
    | ((instr & 0x3ff00000) >> 20) as u32 // imm[10:1]
    | ((instr & 0x80000) >> 9) as u32 // imm[11]
    | (instr & 0xff000) as u32; // imm[19:12]
}

pub fn get_instr_name(instr: u32) -> String {
    let opcode = instr & 0x7f;
    let funct3 = (instr >> 12) & 0x7;
    let funct7 = (instr >> 25) & 0x7f;

    match opcode {
        LUI => "lui".to_string(),
        AUIPC => "auipc".to_string(),
        JAL => "jal".to_string(),
        JALR => "jalr".to_string(),
        B_TYPE => match funct3 {
            BEQ => "beq".to_string(),
            BNE => "bne".to_string(),
            BLT => "blt".to_string(),
            BGE => "bge".to_string(),
            BLTU => "bltu".to_string(),
            BGEU => "bgeu".to_string(),
            _ => panic!(),
        },
        LOAD => match funct3 {
            LB => "lb".to_string(),
            LH => "lh".to_string(),
            LW => "lw".to_string(),
            LBU => "lbu".to_string(),
            LHU => "lhu".to_string(),
            LWU => "lwu".to_string(),
            _ => panic!(),
        },
        S_TYPE => match funct3 {
            SB => "sb".to_string(),
            SH => "sh".to_string(),
            SW => "sw".to_string(),
            _ => panic!(),
        },
        I_TYPE => match funct3 {
            ADDI => "addi".to_string(),
            SLLI => "slli".to_string(),
            SLTI => "slti".to_string(),
            SLTIU => "sltiu".to_string(),
            XORI => "xori".to_string(),
            SRI => match funct7 {
                SRLI => "srli".to_string(),
                SRAI => "srai".to_string(),
                _ => panic!(),
            },
            ORI => "ori".to_string(),
            ANDI => "andi".to_string(),
            _ => {
                panic!("malformed I type instruction");
            }
        },
        R_TYPE => match funct3 {
            ADDSUB => match funct7 {
                ADD => "add".to_string(),
                SUB => "sub".to_string(),
                _ => panic!("errors in ADD/SUB"),
            },
            SLL => "sll".to_string(),
            SLT => "slt".to_string(),
            SLTU => "sltu".to_string(),
            XOR => "xor".to_string(),
            SR => match funct7 {
                SRL => "srl".to_string(),
                SRA => "sra".to_string(),
                _ => panic!("errors in SR"),
            },
            OR => "or".to_string(),
            AND => "and".to_string(),
            _ => {
                panic!("malformed I type instruction");
            }
        },
        FENCE => "fence".to_string(),
        CSR => match (funct3) {
            ECALL | EBREAK => match imm_i(instr) {
                0x0 => "ecall".to_string(),
                0x1 => "ebreak".to_string(),
                _ => "not ECALL/EBREAK".to_string(),
            },
            CSRRW => "csrrw".to_string(),
            CSRRS => "csrrs".to_string(),
            CSRRC => "csrrc".to_string(),
            CSRRWI => "csrrwi".to_string(),
            CSRRSI => "csrrsi".to_string(),
            CSRRCI => "csrrci".to_string(),
            _ => {
                panic!("malformed CSR instruction");
            }
        },
        _ => panic!("invalid instr {}, opcode: {:b}", instr, opcode),
    }
}
