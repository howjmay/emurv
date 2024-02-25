use clap::Parser;

use riscland::cpu;
use riscland::elf;
use riscland::opcode::get_instr_name;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    // input binary file
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut cpu = cpu::CPU::new();
    let elf_file = elf::ELF::new(&args.file);
    let file_bin = elf_file.read_instructions_to_end();
    cpu.bus.init_memory(file_bin);
    let mut cnt = 0;
    loop {
        let instr = cpu.fetch();
        println!(
            "cnt: {}, cpu.pc: {:#x}, instr: {:x}, name: {}",
            cnt,
            cpu.pc,
            instr,
            get_instr_name(instr),
        );
        cnt += 1;
        cpu.execute(instr);
        cpu.pc += 4;
        // riscland::debug::dump_registers(&cpu);
    }
}
