#[cfg(test)]
mod tests {
    use emurv::cpu;
    #[test]
    fn test_exec_addi() {
        let mut cpu_test = cpu::CPU::new();
        let instr: Vec<u8> = vec![
            0x93, 0x0F, 0x40, 0x00, // addi x31, x0, 4
        ];
        cpu::exec_addi(&mut cpu_test, u32::from_le_bytes(instr.try_into().unwrap()));
        assert_eq!(cpu_test.xregs.regs[31], 4);
    }
}
