use clap::Parser;

use riscland::cpu;
use riscland::files;

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
    let file_bin = files::read_file(&args.file);
    cpu.bus.init_memory(file_bin);
}
