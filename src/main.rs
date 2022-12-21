struct CPU {
    current_op: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_op
    }

    fn run(&self) {
        // loop {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add(x, y),
            _ => todo!("opcode {opcode:04x}"),
        };
        // }
    }

    fn add(&self, x: u8, y: u8) -> u8 {
        self.registers[x as usize] + self.registers[y as usize]
    }
}

fn main() {
    let mut cpu = CPU {
        current_op: 0,
        registers: [0; 2],
    };

    cpu.current_op = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();
}
