use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, Sdl};
use std::fs;

const SCREEN_SCALE: usize = 20;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;
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
/// The chip8 processor abstraction
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

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let sdl_ctx = sdl2::init().unwrap();
        let video_subsys = sdl_ctx.video().unwrap();

        let g_window = video_subsys
            .window(
                "Chip 8 Emulator",
                (SCREEN_WIDTH * SCREEN_SCALE) as u32,
                (SCREEN_HEIGHT * SCREEN_SCALE) as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let render = g_window.into_canvas().accelerated().build().unwrap();
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
        chip8.cls();
        chip8.load_font();
        chip8
    }
    fn cls(&mut self) {
        for i in 0..SCREEN_SIZE {
            self.screen[i] = 0x01;
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
        self.g_render.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        self.g_render.clear();
        self.g_render.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if (self.screen[x + (y * SCREEN_WIDTH)]) != 0 {
                    self.g_render
                        .fill_rect(Rect::new(
                            (x * SCREEN_SCALE) as i32,
                            (y * SCREEN_SCALE) as i32,
                            SCREEN_SCALE as u32,
                            SCREEN_SCALE as u32,
                        ))
                        .unwrap();
                }
            }
        }
        self.g_render.present();
        self.should_draw = false;
    }

    fn update_opcode(&mut self) {
        self.opcode = self.memory[(self.pc) as usize] as u16;
        self.opcode <<= 8;
        self.opcode |= self.memory[(self.pc + 1) as usize] as u16;
    }

    fn get_opcode_nibbles(&self) -> (u8, u8, u8, u8) {
        (
            ((self.opcode & 0xf000) >> 12) as u8,
            ((self.opcode & 0x0f00) >> 8) as u8,
            ((self.opcode & 0x00f0) >> 4) as u8,
            (self.opcode & 0x000f) as u8,
        )
    }

    pub fn run(&mut self) {
        self.update_opcode();
        let nibble = self.get_opcode_nibbles();
        let nnn = self.opcode & 0x0fff;
        let kk = (self.opcode & 0x00ff) as u8;
        let x = nibble.1 as usize;
        let y = nibble.2 as usize;
        let n = nibble.3;
        match nibble.0 {
            0x0 => match nibble.3 {
                0x0 => {
                    self.cls();
                    self.should_draw = true;
                    self.pc += 2;
                }
                0xe => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                }
                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            0x1 => {
                self.pc = nnn;
            }
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3 => {
                if self.v_reg[x] == kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x4 => {
                if self.v_reg[x] != kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x5 => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6 => {
                self.v_reg[x] = kk;
                self.pc += 2;
            }
            0x7 => {
                self.v_reg[x] += kk;
                self.pc += 2;
            }
            0x8 => match nibble.3 {
                0 => {
                    self.v_reg[x] = self.v_reg[y];
                    self.pc += 2;
                }
                1 => {
                    self.v_reg[x] |= self.v_reg[y];
                    self.pc += 2;
                }
                2 => {
                    self.v_reg[x] &= self.v_reg[y];
                    self.pc += 2;
                }
                3 => {
                    self.v_reg[x] ^= self.v_reg[y];
                    self.pc += 2;
                }
                4 => {
                    let sum = (self.v_reg[x] + self.v_reg[y]) as u16;
                    self.v_reg[x] = (sum & 0x00ff) as u8;
                    if sum > 0xff {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.pc += 2;
                }
                5 => {
                    if self.v_reg[x] > self.v_reg[y] {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[x] -= self.v_reg[y];
                    self.pc += 2;
                }
                6 => {
                    if (self.v_reg[x] & 0x01) == 0x01 {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[x] >>= 1;
                    self.pc += 2;
                }
                7 => {
                    if self.v_reg[y] > self.v_reg[x] {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[x] = self.v_reg[y] - self.v_reg[x];
                    self.pc += 2;
                }
                0xe => {
                    if (self.v_reg[x] & 0x80) == 0x80 {
                        self.v_reg[0xf] = 1;
                    } else {
                        self.v_reg[0xf] = 0;
                    }
                    self.v_reg[x] <<= 1;
                    self.pc += 2;
                }

                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            0x9 => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0xa => {
                self.i = nnn;
                self.pc += 2;
            }
            0xb => {
                self.pc = (self.v_reg[0] as u16) + nnn;
                self.pc += 2;
            }
            0xc => {
                let mut rng = rand::thread_rng();
                let rand_num = rng.gen::<u8>() & kk;
                self.v_reg[x] = rand_num;
                self.pc += 2;
            }
            0xd => {
                let pos_x = x;
                let pos_y = y;
                let sprite_size = n;
                self.v_reg[0xf] = 0x00;
                for y_line in 0..sprite_size {
                    let pixel = (self.memory[(self.i + y_line as u16) as usize]) as u16;
                    for x_line in 0..8 {
                        if pixel & (0x80 >> x_line) != 0 {
                            let screen_pos =
                                pos_x + x_line + (pos_y + y_line as usize) * SCREEN_WIDTH;
                            self.v_reg[0xf] = self.screen[screen_pos];
                            self.screen[screen_pos] ^= 1;
                        }
                    }
                }
                self.pc += 2;
                self.should_draw = true;
            }
            0xe => match kk {
                0x009e => {
                    let value = self.v_reg[x];
                    if self.keyboard[value as usize] != 0 {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                0x00a1 => {
                    let value = self.v_reg[x];
                    if self.keyboard[value as usize] == 0 {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                opcode => {
                    eprintln!("Encountered unrecognised Opcode: {opcode:#06X}");
                }
            },
            0xf => match kk {
                0x07 => {
                    self.v_reg[x] = self.delay_timer;
                    self.pc += 2;
                }
                0x0a => {
                    let mut key_press = false;
                    let mut i: u8 = 0;
                    while i < 16 {
                        if self.keyboard[i as usize] != 0 {
                            self.v_reg[x] = i;
                            key_press = true;
                        }
                        i += 1;
                    }
                    if key_press {
                        self.pc += 2;
                    } else {
                        return;
                    }
                }
                0x15 => {
                    self.delay_timer = self.v_reg[x];
                    self.pc += 2;
                }
                0x18 => {
                    self.sound_timer = self.v_reg[x];
                    self.pc += 2;
                }
                0x1e => {
                    self.i += self.v_reg[x] as u16;
                    self.pc += 2;
                }
                0x33 => {
                    let mut value = self.v_reg[x];
                    self.memory[self.i as usize] = value % 10;
                    value /= 10;
                    self.memory[(self.i + 1) as usize] = value % 10;
                    value /= 10;
                    self.memory[(self.i + 2) as usize] = value % 10;
                    self.pc += 2;
                }
                0x29 => {
                    self.i = (self.v_reg[x] as u16) * 0x05;
                    self.pc += 2;
                }
                0x55 => {
                    let mut i = 0;
                    while i <= x {
                        self.memory[(self.i + i as u16) as usize] = self.v_reg[i];
                        i += 1;
                    }
                    self.pc += 2;
                }
                0x65 => {
                    let mut i = 0;
                    while i <= x {
                        self.v_reg[i] = self.memory[(self.i + i as u16) as usize];
                        i += 1;
                    }
                    self.pc += 2;
                }
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
        //self._print_details();
    }
    fn _print_details(&self) {
        println!("[ I = {} ]", self.i);
    }
}
