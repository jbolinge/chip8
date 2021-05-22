#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    pub program_counter: u16,
    pub register_v: [u8; 0xF],
    pub register_i: u16,
    pub stack: Vec<u16>,
    pub ram: [u8; 0xFFF],
    pub display_buffer: [u8; 64 * 32],
    pub delay: u8,
    pub sound: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0x0000,
            register_v: [0x00; 0xF],
            register_i: 0x0000,
            stack: Vec::with_capacity(0x40),
            ram: [0x00; 0xFFF],
            display_buffer: [0x00; 64 * 32],
            delay: 0x00,
            sound: 0x00,
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        loop {
            let opscode: u16 = ((program[self.program_counter as usize] as u16) << 8)
                | program[(self.program_counter + 1) as usize] as u16;
            self.program_counter += 2; // Need to increment by 2 as each opscode is 16 bits
            match opscode >> 12 {
                0x0 => {
                    match opscode {
                        0x00E0 => self.clear_buffer(),
                        0x00EE => self.ret(),
                        _ => self.sys(opscode),
                    };
                }
                0x1 => self.jp(opscode),
                0x2 => self.call(opscode),
                // Match on the first 4 bits
                0xA => self.ldi(opscode),
                _ => break,
            }
            if self.program_counter as usize >= program.len() {
                break;
            }
        }
    }

    fn ldi(&mut self, opscode: u16) {
        self.register_i = opscode & 0x0FFF;
    }

    fn call(&mut self, opscode: u16) {
        self.stack.push(self.program_counter);
        self.program_counter = opscode & 0x0FFF;
    }

    fn ret(&mut self) {
        self.program_counter = self.stack.pop().unwrap();
    }

    fn jp(&mut self, opscode: u16) {
        self.program_counter = opscode & 0x0FFF;
    }

    #[allow(unused_variables)]
    fn sys(&mut self, opscode: u16) {
        // Should be ignored by modern compilers
    }

    fn clear_buffer(&mut self) {
        for pixel in self.display_buffer.iter_mut() {
            *pixel = 0x00;
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU::new()
    }
}

mod test {

    use super::*;

    #[test]
    fn test_ldi() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xA2, 0x34, 0xFF, 0xFF]);
        assert_eq!(cpu.register_i, 0x0234);
    }

    #[test]
    fn test_call() {
        let mut cpu = CPU::new();
        cpu.run(vec![0x20, 0x04, 0xFF, 0xFF, 0xA1, 0x23, 0xFF, 0xFF]);
        assert_eq!(cpu.register_i, 0x0123);
        assert_eq!(cpu.stack.len(), 1);
    }

    #[test]
    fn test_ret() {
        let mut cpu = CPU::new();
        cpu.run(vec![
            0x20, 0x06, 0xA1, 0x23, 0xFF, 0xFF, 0xA4, 0x56, 0x00, 0xEE, 0xFF, 0xFF,
        ]);
        assert_eq!(cpu.register_i, 0x0123);
        assert_eq!(cpu.stack.len(), 0);
    }
}
