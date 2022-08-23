use rand::Rng;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::WindowCanvas, Sdl,
};
use std::fs;

const SCREEN_SCALE: usize = 1;
const SCREEN_HEIGHT: usize = 32 * SCREEN_SCALE;
const SCREEN_WIDTH: usize = 64 * SCREEN_SCALE;
const SCREEN_SIZE: usize = SCREEN_HEIGHT * SCREEN_WIDTH;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
pub struct Chip8 {
    i: u16,
    pc: u16,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    opcode: u16,
    stack: [u16; 16],
    v_reg: [u8; 16],
    memory: [u8; 4096],
    keyboard: [u8; 16],
    screen: [u8; SCREEN_SIZE],
    g_render: WindowCanvas,
    g_sdl: Sdl,
    should_quit: bool,
    should_draw: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let sdl_ctx = sdl2::init().unwrap();
        let video_subsys = sdl_ctx.video().unwrap();

        let g_window = video_subsys
            .window("Chip 8 Emulator", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut render = g_window.into_canvas().build().unwrap();
        render
            .set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32)
            .unwrap();
        let mut chip8 = Chip8 {
            i: 0x0000,
            pc: 0x0200,
            sp: 0x00,
            delay_timer: 0x00,
            sound_timer: 0x00,
            opcode: 0x0000,
            stack: [0x0000; 16],
            v_reg: [0x00; 16],
            memory: [0x00; 4096],
            keyboard: [0x00; 16],
            screen: [0x00; SCREEN_SIZE],
            g_render: render,
            g_sdl: sdl_ctx,
            should_quit: false,
            should_draw: false,
        };
        chip8.graphics_init();
        chip8.cls();
        chip8.load_font();
        chip8
    }
    fn cls(&mut self) {
        let mut i = 0;
        while i < SCREEN_SIZE {
            self.screen[i] = 0x00;
            i += 1;
        }
    }
    fn load_font(&mut self) {
        let mut i = 0;
        while i < 80 {
            self.memory[i] = FONTSET[i];
            i += 1;
        }
    }
    pub fn load_rom(&mut self, rom_path: String) {
        let content = fs::read(rom_path).expect("Erroring reading rom");
        for (i, el) in content.iter().enumerate() {
            self.memory[(self.pc + i as u16) as usize] = *el;
        }
    }
    fn graphics_init(&mut self) {}
    pub fn quitting(&self) -> bool {
        self.should_quit
    }
    pub fn should_render(&self) -> bool {
        self.should_draw
    }
    pub fn capture_keys(&mut self) {
        let mut g_event = self.g_sdl.event_pump().unwrap();
        for event in g_event.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.should_quit = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.keyboard[0x01] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    self.keyboard[0x02] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    self.keyboard[0x03] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.keyboard[0x04] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    self.keyboard[0x05] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    self.keyboard[0x06] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => {
                    self.keyboard[0x07] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    self.keyboard[0x08] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    self.keyboard[0x09] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    self.keyboard[0x0a] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    self.keyboard[0x0b] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => {
                    self.keyboard[0x0c] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => {
                    self.keyboard[0x0d] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => {
                    self.keyboard[0x0e] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => {
                    self.keyboard[0x0f] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => {
                    self.keyboard[0x00] = 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.keyboard[0x01] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    self.keyboard[0x02] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    self.keyboard[0x03] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.keyboard[0x04] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    self.keyboard[0x05] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    self.keyboard[0x06] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num0),
                    ..
                } => {
                    self.keyboard[0x07] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    self.keyboard[0x08] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    self.keyboard[0x09] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    self.keyboard[0x0a] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    self.keyboard[0x0b] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num5),
                    ..
                } => {
                    self.keyboard[0x0c] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num6),
                    ..
                } => {
                    self.keyboard[0x0d] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num7),
                    ..
                } => {
                    self.keyboard[0x0e] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num8),
                    ..
                } => {
                    self.keyboard[0x0f] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num9),
                    ..
                } => {
                    self.keyboard[0x00] = 0;
                }
                _ => {}
            }
        }
    }
    pub fn draw(&mut self) {
        self.g_render.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
        self.g_render.clear();
        self.g_render.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        let mut y = 0;
        while y < SCREEN_HEIGHT {
            let mut x = 0;
            while x < SCREEN_WIDTH {
                let row_num = y * SCREEN_WIDTH;
                if self.screen[x + row_num] != 0 {
                    match self.g_render.draw_point(Point::new(x as i32, y as i32)) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                x += 1;
            }
            y += 1;
        }
        self.g_render.present();
        self.should_draw = false;
    }
    pub fn run(&mut self) {
        self.opcode = self.memory[(self.pc) as usize] as u16;
        self.opcode <<= 8;
        self.opcode |= self.memory[(self.pc + 1) as usize] as u16;
        match self.opcode & 0xf000 {
            0x0000 => match self.opcode & 0x000f {
                0x0000 => {
                    self.cls();
                    self.should_draw = true;
                    self.pc += 2;
                }
                0x000e => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                }
                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            0x1000 => {
                self.pc = self.opcode & 0x0fff;
            }
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0fff;
            }
            0x3000 => {
                let reg_index = ((self.opcode & 0x0f00) >> 8) as u8;
                let value = (self.opcode & 0x00ff) as u8;
                if self.v_reg[reg_index as usize] == value {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x4000 => {
                let reg_index = ((self.opcode & 0x0f00) >> 8) as u8;
                let value = (self.opcode & 0x00ff) as u8;
                if self.v_reg[reg_index as usize] != value {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x5000 => {
                let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                if self.v_reg[reg_x as usize] == self.v_reg[reg_y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6000 => {
                let reg_index = ((self.opcode & 0x0f00) >> 8) as u8;
                let value = (self.opcode & 0x00ff) as u8;
                self.v_reg[reg_index as usize] = value;
                self.pc += 2;
            }
            0x7000 => {
                let reg_index = ((self.opcode & 0x0f00) >> 8) as u8;
                let value = (self.opcode & 0x00ff) as u8;
                self.v_reg[reg_index as usize] += value;
                self.pc += 2;
            }
            0x8000 => match self.opcode & 0x000f {
                0 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                    self.v_reg[reg_x as usize] = self.v_reg[reg_y as usize];
                    self.pc += 2;
                }
                1 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                    self.v_reg[reg_x as usize] |= self.v_reg[reg_y as usize];
                    self.pc += 2;
                }
                2 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                    self.v_reg[reg_x as usize] &= self.v_reg[reg_y as usize];
                    self.pc += 2;
                }
                3 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                    self.v_reg[reg_x as usize] ^= self.v_reg[reg_y as usize];
                    self.pc += 2;
                }
                4 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u16;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u16;
                    let sum = (self.v_reg[reg_x as usize] + self.v_reg[reg_y as usize]) as u16;
                    self.v_reg[reg_x as usize] = (sum & 0xff) as u8;
                    if sum > 0xff {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.pc += 2;
                }
                5 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                    if self.v_reg[reg_x as usize] > self.v_reg[reg_y as usize] {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.pc += 2;
                }
                6 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    if (self.v_reg[reg_x as usize] & 0x01) == 0x01 {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[reg_x as usize] >>= 1;
                    self.pc += 2;
                }
                7 => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                    if self.v_reg[reg_y as usize] > self.v_reg[reg_x as usize] {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[reg_x as usize] =
                        self.v_reg[reg_y as usize] - self.v_reg[reg_x as usize];
                    self.pc += 2;
                }
                0x0e => {
                    let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                    if (self.v_reg[reg_x as usize] & 0x80) == 0x80 {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[reg_x as usize] <<= 1;
                    self.pc += 2;
                }

                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            0x9000 => {
                let reg_x = ((self.opcode & 0x0f00) >> 8) as u8;
                let reg_y = ((self.opcode & 0x00f0) >> 4) as u8;
                if self.v_reg[reg_x as usize] != self.v_reg[reg_y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0xa000 => {
                self.i = self.opcode & 0x0fff;
                self.pc += 2;
            }
            0xb000 => {
                self.pc = (self.v_reg[0] + (self.opcode & 0x0fff) as u8) as u16;
                self.pc += 2;
            }
            0xc000 => {
                let reg_index = ((self.opcode & 0x0f00) >> 8) as u8;
                let value = (self.opcode & 0x00ff) as u8;
                let mut rng = rand::thread_rng();
                self.v_reg[reg_index as usize] = rng.gen_range(0..255) & value;
                self.pc += 2;
            }
            0xd000 => {
                let pos_x = self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] as u16;
                let pos_y = self.v_reg[((self.opcode & 0x00f0) >> 4) as usize] as u16;
                let sprite_size = (self.opcode & 0x000f) as u16;
                let mut pixel: u16;
                self.v_reg[0xf] = 0x00;
                let mut y_line = 0;
                while y_line < sprite_size {
                    pixel = (self.memory[(self.i + y_line as u16) as usize]) as u16;
                    let mut x_line = 0;
                    while x_line < 8 {
                        if (pixel & (0x80 >> x_line)) == 0 {
                            if self.screen[(pos_x + x_line + ((pos_y + y_line) * 64)) as usize] == 1
                            {
                                self.v_reg[0xf] = 1;
                            }
                        }
                        self.screen[(pos_x + x_line + ((pos_y + y_line) * 64)) as usize] ^= 1;
                        x_line += 1;
                    }
                    y_line += 1;
                }
                self.pc += 2;
                self.should_draw = true;
            }
            0xe000 => match self.opcode & 0x00ff {
                0x009e => {
                    let value = self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] as u8;
                    if self.keyboard[value as usize] != 0 {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                0x00a1 => {
                    let value = self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] as u8;
                    if self.keyboard[value as usize] == 0 {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            0xf000 => match self.opcode & 0x00ff {
                0x0007 => {
                    self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] = self.delay_timer;
                    self.pc += 2;
                }
                0x000a => {
                    let mut key_press = false;
                    let mut i: u8 = 0;
                    while i < 16 {
                        if self.keyboard[i as usize] != 0 {
                            self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] = i;
                            key_press = true;
                        }
                        i += 1;
                    }
                    if key_press {
                        self.pc += 2;
                    } else {
                        return;
                    }
                },
                0x0015 => {
                    self.delay_timer = self.v_reg[((self.opcode & 0x0f00) >> 8) as usize];
                    self.pc += 2;
                },
                0x0018 => {
                    self.sound_timer = self.v_reg[((self.opcode & 0x0f00) >> 8) as usize];
                    self.pc += 2;
                },
                0x001e => {
                    self.i += self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] as u16;
                    self.pc += 2;
                },
                0x0033 => {
                    let mut value = self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] as u8;
                    let mut i =3;
                    while i >= 1 {
                        let bcd = value % 10;
                        self.memory[(self.i + i) as usize] = bcd;
                        i -= 1;
                        value /= 10;
                    }
                },
                0x0029 => {
                    self.i = (self.v_reg[((self.opcode & 0x0f00) >> 8) as usize] * 0x05) as u16;
                    self.pc +=2;
                },
                0x0055 =>{
                    let limit = (self.opcode & 0x0f00) >> 8;
                    let mut i = 0;
                    while i <= limit {
                        self.memory[(self.i + i) as usize] =self.v_reg[i as usize];
                        i +=1;
                    }
                    self.pc += 2;
                },
                0x0065 =>{
                    let limit = (self.opcode & 0x0f00) >> 8;
                    let mut i = 0;
                    while i <= limit {
                        self.v_reg[i as usize] = self.memory[(self.i + i) as usize];
                        i +=1;
                    }
                    self.pc += 2;

                },
                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            opcode => {
                eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
            }
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            print!(".");
            self.sound_timer -= 1;
        }
    }
}
