struct CPU {
    registers: [u8; 16],
    program_counter: usize, // position in memory
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn run(&mut self) {
        loop {
            let p = self.program_counter;

            let op_byte1 = self.memory[p] as u16;
            let op_byte2 = self.memory[p + 1] as u16;
            let opcode = (op_byte1 << 8) | op_byte2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;

            let op_minor = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            self.program_counter += 2; // 1 opcode = 2 u8

            match (opcode) {
                0x0000 => return,
                0x00E0 => { /* CLRSCR */ }
                0x00EE => self.ret(),
                0x1000..=0x1FFF => todo!("jump to addr"),
                0x2000..=0x2FFF => self.call(addr),
                0x3000..=0x3FFF => todo!("set if equal"),
                0x4000..=0x4FFF => todo!("set if not equal"),
                0x5000..=0x5FFF => todo!("set if equal"),
                0x6000..=0x6FFF => todo!("LD set kk to reg vk"),
                0x7000..=0x7FFF => todo!("adds kk to reg vk"),
                0x8000..=0x8FFF => match op_minor {
                    0 => todo!("set x to reg y"),
                    1 => todo!("x or y"),
                    2 => todo!("x & y"),
                    3 => todo!("x xor y"),
                    4 => todo!("x+y"),
                    _ => todo!("opcode: {:04x}", opcode),
                },
                _ => todo!("opcode {:04x}", opcode),
            };
        }
    }

    fn call(&mut self, addr: u16) {
        let stack_ptr = self.stack_pointer;
        let stack = &mut self.stack;

        if stack_ptr > stack.len() {
            panic!("Stack overflow!")
        }

        self.stack[stack_ptr] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // last register of CHIP-8 is a carry flag.
        // If set indicates that an operation has overflowed the u8 register size
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
