
    extern crate rand;
    use rand::Rng;



    #[derive(Copy,Clone)]
    struct Pixel {
        color: [u8;4],
    }

    impl Pixel {
        fn new() -> Self {
            Self {
                color: [0, 0xFF, 0xFF, 0xFF],
            }
        }

        fn new_random() -> Self {
            let mut pixel : Pixel = Pixel::new();
            pixel.randomize();
            pixel
        }

        fn randomize(&mut self) {
            self.color = rand::thread_rng().gen();
        }

    }

    #[derive(Copy,Clone)]
    struct Sprite {
        pixels: [[Pixel;crate::SPRITE_WIDTH];crate::SPRITE_HEIGHT],
    }

    impl Sprite {
        fn new() -> Self {
            Self {
                pixels: [[Pixel::new();crate::SPRITE_WIDTH];crate::SPRITE_HEIGHT],
            }
        }
        fn new_random() -> Self {
            Self {
                pixels: [[Pixel::new_random();crate::SPRITE_WIDTH];crate::SPRITE_HEIGHT],
            }
        }
    }

    struct CPU {
        registers: [u8; 16],
        position_in_memory: usize,
        memory: [u8; 0x10000],
        stack: [u16; 16],
        stack_pointer: usize,
    }

    impl CPU {
        fn new() -> Self {
            Self {
                registers: [0; 16],
                memory: [0; 65536],
                position_in_memory: 0,
                stack: [0; 16],
                stack_pointer: 0,
            }
        }

        fn read_opcode(&self) -> u16 {
            let p = self.position_in_memory;
            let op_byte1 = self.memory[p] as u16;
            let op_byte2 = self.memory[p + 1] as u16;

            op_byte1 << 8 | op_byte2
        }

        fn run(&mut self) {
            loop {
                let opcode = self.read_opcode();
                self.position_in_memory += 2;

                let c = ((opcode & 0xF000) >> 12) as u8;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let d = ((opcode & 0x000F) >> 0) as u8;

                let nnn = opcode & 0x0FFF;
                match (c, x, y, d) {
                    (0, 0, 0, 0) => { return; }
                    (0, 0, 0xE, 0xE) => self.ret(),
                    (0x8, _, _, 0x4) => self.add_xy(x, y),
                    (0x2, _, _, _) => self.call(nnn),
                    _ => todo!("opcode {0:04x}", opcode),
                }
            }
        }

        fn call(&mut self, addr: u16) {
            let sp = self.stack_pointer;
            let stack = &mut self.stack;

            if sp > stack.len()
            {
                panic!("Stack Overflow");
            }

            stack[sp] = self.position_in_memory as u16;
            self.stack_pointer += 1;
            self.position_in_memory = addr as usize;
        }

        fn ret(&mut self) {
            if self.stack_pointer == 0 {
                panic!("Stack underflow");
            }

            self.stack_pointer -= 1;
            let call_addr = self.stack[self.stack_pointer];
            self.position_in_memory = call_addr as usize;
        }

        fn add_xy(&mut self, x: u8, y: u8)
        {
            let arg1 = self.registers[x as usize];
            let arg2 = self.registers[y as usize];

            let (val, overflow) = arg1.overflowing_add(arg2);
            self.registers[x as usize] = val;

            if overflow {
                self.registers[0xF] = 1;
            } else {
                self.registers[0xF] = 0;
            }
        }
    }

     pub struct GameBoy {
            map:Vec<Vec<Sprite>>,
     }

    impl GameBoy{
        pub fn new() -> Self {
            let mut ranvec:Vec<Sprite> = Vec::with_capacity(crate::MAP_WIDTH);
            for _ in 0..crate::MAP_WIDTH {
                ranvec.push(Sprite::new_random());
            }
            Self {
                map: vec![ranvec;crate::MAP_HEIGHT],
            }
        }

        fn get_flat_map(&self) -> [Pixel;crate::SCREEN_SIZE] {
            let mut screen = [Pixel::new();crate::SCREEN_SIZE];


            for i in 0..crate::SCREEN_HEIGHT {
                for j in 0..crate::SPRITE_HEIGHT {
                    for k in 0..crate::SCREEN_WIDTH {
                        for l in 0..crate::SPRITE_WIDTH {
                            let val = i*crate::SPRITE_HEIGHT*crate::SCREEN_WIDTH*crate::SPRITE_WIDTH + j * crate::SCREEN_WIDTH*crate::SPRITE_WIDTH + k * crate::SPRITE_WIDTH + l;
                            screen[val] = self.map[i][k].pixels[j][l].clone();
                        }
                    }
                }
            }

            screen
        }

        pub fn draw(&self, screen: &mut [u8]) {
            let flat_map = self.get_flat_map();
            for (c, pix) in flat_map.iter().zip(screen.chunks_exact_mut(4)) {
                pix.copy_from_slice(&c.color);
            }
        }
    }
