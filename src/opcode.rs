// pub mod opcode;

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
