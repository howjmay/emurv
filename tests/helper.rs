use emurv::cpu;

pub fn set_i_type_instruction(imm: i16, rs1: u8, funct3: u8, rd: u8, opcode: u8) -> u32 {
    // |31-20|19-15|14-12|11-7|6-0|
    return ((imm as u32 & 0xfff) << 20)
        | ((rs1 as u32 & 0x1f) << 15)
        | ((funct3 as u32 & 0x7) << 12)
        | ((rd as u32 & 0x1f) << 7)
        | ((opcode as u32) & 0x7f);
}

pub fn set_register_val(cpu: &mut cpu::CPU, rd: u8, val: i16) {
    let instr = set_i_type_instruction(val, 0x0, 0x0, rd, 0x13);
    cpu::exec_addi(cpu, instr);
}
