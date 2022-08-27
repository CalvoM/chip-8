use std::time::Duration;

use r_emulator::chip8::Chip8;

pub fn main() {
    let mut chip = Chip8::new();
    chip.load_rom(String::from("../roms/chip-8/MAZE"));
    while !chip.quitting() {
        chip.run();
        if chip.should_render() {
            chip.draw();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
        chip.capture_keys();
    }
}
