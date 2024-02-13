mod helper;

#[cfg(test)]
mod tests {
    use crate::helper;
    use emurv::{cpu, opcode::*};

    #[test]
    fn test_exec_lui() {
        let mut cpu_test = cpu::CPU::new();

        // lui x5, 4
        let instr: u32 = helper::set_u_type_instruction(4 << 12, 5, LUI as u8);
        cpu::exec_lui(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[5], 4 << 12);

        // lui x5, -4
        let instr: u32 = helper::set_u_type_instruction(-4 << 12, 5, LUI as u8);
        cpu::exec_lui(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[5], (-4 << 12) as u32);
    }
    #[test]
    fn test_exec_auipc() {
        let mut cpu_test = cpu::CPU::new();

        let ori_pc = cpu_test.pc;
        // auipc x5, 4
        let instr: u32 = helper::set_u_type_instruction(4 << 12, 5, AUIPC as u8);
        cpu::exec_auipc(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[5], ori_pc + (4 << 12));

        // auipc x5, -4
        let instr: u32 = helper::set_u_type_instruction(-4 << 12, 5, AUIPC as u8);
        cpu::exec_auipc(&mut cpu_test, instr);
        assert_eq!(
            cpu_test.xregs.regs[5],
            (ori_pc as i32).wrapping_add(-4 << 12) as u32
        );
    }
    #[test]
    fn test_exec_jal() {
        let mut cpu_test = cpu::CPU::new();

        let ori_pc = cpu_test.pc;
        // jal x5, 12
        let instr: u32 = helper::set_j_type_instruction(12, 5, JAL as u8);
        cpu::exec_jal(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[5], ori_pc + 4);
        assert_eq!(cpu_test.pc, ori_pc + 12);
    }
    #[test]
    fn test_exec_jalr() {}
    #[test]
    fn test_exec_beq() {}
    #[test]
    fn test_exec_bne() {}
    #[test]
    fn test_exec_blt() {}
    #[test]
    fn test_exec_bge() {}
    #[test]
    fn test_exec_bltu() {}
    #[test]
    fn test_exec_bgeu() {}
    #[test]
    fn test_exec_lb() {}
    #[test]
    fn test_exec_lh() {}
    #[test]
    fn test_exec_lw() {}
    #[test]
    fn test_exec_ld() {}
    #[test]
    fn test_exec_lbu() {}
    #[test]
    fn test_exec_lhu() {}
    #[test]
    fn test_exec_lwu() {}
    #[test]
    fn test_exec_sb() {}
    #[test]
    fn test_exec_sh() {}
    #[test]
    fn test_exec_sw() {}
    #[test]
    fn test_exec_sd() {}
    #[test]
    fn test_exec_addi() {
        let mut cpu_test = cpu::CPU::new();

        // addi x31, x0, 4
        let instr: u32 = helper::set_i_type_instruction(4, 0, ADDI as u8, 31);
        cpu::exec_addi(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 4);
    }
    #[test]
    fn test_exec_slti() {
        let mut cpu_test = cpu::CPU::new();

        // set x1=3
        helper::set_register_val(&mut cpu_test, 1, 3);
        // slti x31, x1, 2
        let instr: u32 = helper::set_i_type_instruction(2, 1, SLTI as u8, 31);
        cpu::exec_slti(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 0);
        // slti x31, x1, 4
        let instr: u32 = helper::set_i_type_instruction(4, 1, SLTI as u8, 31);
        cpu::exec_slti(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 1);
        // slti x31, x1, -2
        let instr: u32 = helper::set_i_type_instruction(-2, 1, SLTI as u8, 31); // 254 == -2
        cpu::exec_slti(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 0);
    }
    #[test]
    fn test_exec_sltiu() {
        let mut cpu_test = cpu::CPU::new();

        // set x1=3
        helper::set_register_val(&mut cpu_test, 1, 3);
        // sltiu x31, x1, 2
        let instr: u32 = helper::set_i_type_instruction(2, 1, SLTIU as u8, 31);
        cpu::exec_sltiu(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 0);
        // sltiu x31, x1, 4
        let instr: u32 = helper::set_i_type_instruction(4, 1, SLTIU as u8, 31);
        cpu::exec_sltiu(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 1);
        // sltiu x31, x1, 254
        let instr: u32 = helper::set_i_type_instruction(-2, 1, SLTIU as u8, 31);
        cpu::exec_sltiu(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 1);
    }
    #[test]
    fn test_exec_xori() {
        let mut cpu_test = cpu::CPU::new();

        // xori x31, x0, 4
        let instr: u32 = helper::set_i_type_instruction(4, 0, XORI as u8, 31);
        cpu::exec_xori(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 4);
    }
    #[test]
    fn test_exec_ori() {
        let mut cpu_test = cpu::CPU::new();

        // ori x31, x0, 4
        let instr: u32 = helper::set_i_type_instruction(4, 0, ORI as u8, 31);
        cpu::exec_ori(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 4);
    }
    #[test]
    fn test_exec_andi() {
        let mut cpu_test = cpu::CPU::new();

        // andi x31, x0, 4
        let instr: u32 = helper::set_i_type_instruction(4, 0, ANDI as u8, 31);
        cpu::exec_andi(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 0);
    }
    #[test]
    fn test_exec_slli() {
        let mut cpu_test = cpu::CPU::new();

        // set x1=3
        helper::set_register_val(&mut cpu_test, 1, 3);
        // slli x31, x0, 2
        let instr: u32 = helper::set_i_type_instruction(2, 1, SLLI as u8, 31);
        cpu::exec_slli(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 12);
    }
    #[test]
    fn test_exec_srli() {
        let mut cpu_test = cpu::CPU::new();

        // set x1=254
        helper::set_register_val(&mut cpu_test, 1, 254);
        // srli x31, x0, 2
        let instr: u32 = helper::set_i_type_instruction(2, 1, SRLI as u8, 31);
        cpu::exec_srli(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 63);
    }
    #[test]
    fn test_exec_srai() {
        let mut cpu_test = cpu::CPU::new();

        // set x1=-2
        helper::set_register_val(&mut cpu_test, 1, -2);
        // srli x31, x0, 2
        let instr: u32 = helper::set_i_type_instruction(2, 1, SRAI as u8, 31);
        cpu::exec_srai(&mut cpu_test, instr);
        // -2 >> 2 = -1
        assert_eq!(cpu_test.xregs.regs[31], std::u32::MAX);
    }
    #[test]
    fn test_exec_add() {
        let mut cpu_test = cpu::CPU::new();

        // set x5=-2
        helper::set_register_val(&mut cpu_test, 5, -2);
        // set x6=4
        helper::set_register_val(&mut cpu_test, 6, 4);
        // add x31, x5, x6
        let instr: u32 = helper::set_r_type_instruction(ADD as u8, 5, 6, 31);
        cpu::exec_add(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 2);
    }
    #[test]
    fn test_exec_sub() {
        let mut cpu_test = cpu::CPU::new();

        // set x5=-2
        helper::set_register_val(&mut cpu_test, 5, -2);
        // set x6=4
        helper::set_register_val(&mut cpu_test, 6, 4);
        // sub x31, x5, x6
        let instr: u32 = helper::set_r_type_instruction(SUB as u8, 6, 5, 31);
        cpu::exec_sub(&mut cpu_test, instr);
        assert_eq!(cpu_test.xregs.regs[31], 0xfffffffa); // 0xfffffffa is -6
    }
    #[test]
    fn test_exec_sll() {}
    #[test]
    fn test_exec_slt() {}
    #[test]
    fn test_exec_sltu() {}
    #[test]
    fn test_exec_xor() {}
    #[test]
    fn test_exec_srl() {}
    #[test]
    fn test_exec_sra() {}
    #[test]
    fn test_exec_or() {}
    #[test]
    fn test_exec_and() {}
    #[test]
    fn test_exec_fence() {}
    #[test]
    fn test_exec_fence_i() {}
    #[test]
    fn test_exec_ecall() {}
    #[test]
    fn test_exec_ebreak() {}
    #[test]
    fn test_exec_csrrw() {}
    #[test]
    fn test_exec_csrrs() {}
    #[test]
    fn test_exec_csrrc() {}
    #[test]
    fn test_exec_csrrwi() {}
    #[test]
    fn test_exec_csrrsi() {}
    #[test]
    fn test_exec_csrrci() {}
}
