#include "chip8.hpp"
#include <iostream>

int main(int argc, char **argv) {
    Chip8 chip8;
    if (chip8.setup_succesful()) {
        chip8.loadRom("../roms/chip-8/PONG2");
        while (!chip8.quit()) {
            chip8.run();
            if (chip8.render_ready()) {
                chip8.draw();
                SDL_Delay(10);
            }
            chip8.capture_keys();
        }
    }
}