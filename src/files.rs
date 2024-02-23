pub fn read_file(filename: &str) -> Vec<u8> {
    return std::fs::read(filename).unwrap();
}

#[cfg(test)]
mod tests {
    use super::read_file;
    #[test]
    fn test_read_file() {
        let file_byte = read_file("./tests/addi.bin");
        for byte in file_byte {
            if byte != 0 {
                println!("{:x}", byte);
            }
        }
    }
}
