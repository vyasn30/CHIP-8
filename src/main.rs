struct CPU {
    registers: [u8; 16], // 16 registers as only one 0x word can be used to point any register
    position_in_memory: usize, // basically our program counter
    memory: [u8; 0x1000] // 2^12 bits of RAM (4096bits = 4Kb)
}

    impl CPU {
        fn read_opcode(&self) -> u16 {
            let position = self.position_in_memory;
            let op_byte_0 = self.memory[position] as u16;
            let op_byte_1 = self.memory[position+1] as u16;

            op_byte_0 << 8 | op_byte_1
        }
       
        fn run (&mut self){
            loop {
                let opcode = self.read_opcode();
                self.position_in_memory+=2;
                let c = ((opcode & 0xF000) >> 12) as u8;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let d: u8 = ((opcode & 0x000F) >> 0) as u8;
                
                println!("{}, {}, {}, {}", c,x,y,d);

                match (c, x, y, d) {
                    (0,0,0,0) => {return;},
                    (0x8, _, _, 0x4) => self.add_xy(x, y),
                    _ => {}
                }
            }
        }
        
        fn add_xy(&mut self, x:u8, y:u8) {
            let arg1 = self.registers[x as usize];
            let arg2 = self.registers[y as usize];

            let (val, overflow) = arg1.overflowing_add(arg2);

            self.registers[x as usize] = val;

            if overflow{
                self.registers[0xF]= 1;
            }else {
                self.registers[0xF] = 0;
            }
            
        }

        
    }

fn main(){
    let mut cpu = CPU { registers:[0;16], position_in_memory:0, memory:[0;4096]};

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 20;
    cpu.registers[3] = 30;

    let mem = &mut cpu.memory;

    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;
    
    cpu.run();
    
    println!("{}", cpu.registers[0])
    


}