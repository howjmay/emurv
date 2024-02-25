use object::{Object, ObjectSection};
use std::fs;

pub struct ELF {
    path: String,
}

// for 32bit
pub type INSTRUCTION = u32;

impl ELF {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn read_instructions_to_end(&self) -> Vec<u8> {
        let raw_file = fs::read(&self.path).expect("Failed to read ELF file");

        let file_obj = object::File::parse(&*raw_file).expect("Failed to parse ELF file");

        // Find the .text section
        let text_section = file_obj
            .section_by_name(".text.init")
            .expect("Failed to find the .text section");

        // Get the section data, which contains the machine code instructions
        let data = text_section
            .data()
            .expect("Failed to read .text section data");

        return data.to_vec();
        // for constructing riscv instructions
        // let mut instrs = Vec::<u32>::new();
        // // Iterate over the data and interpret each 4 bytes as a u32
        // // This uses little-endian because of RISCV
        // data.chunks_exact(4).for_each(|chunk| {
        //     let instruction = u32::from_le_bytes(chunk.try_into().expect("Chunk conversion error"));
        //     // println!("Instruction: 0x{:x}", instruction);
        //     instrs.push(instruction)
        // });

        // return instrs;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_instructions_to_end() {
        let elf = super::ELF::new("./tests/rv32ui-p-auipc");
        let file_byte = elf.read_instructions_to_end();
        for byte in file_byte {
            if byte != 0 {
                println!("{:x}", byte);
            }
        }
    }
}
