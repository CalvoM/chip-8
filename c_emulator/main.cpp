#include "chip8.hpp"
#include <iostream>

int main(int argc, char **argv) {
    Chip8 chip8;
    if (chip8.setup_succesful()) {
        chip8.loadRom("../roms/chip-8/MAZE");
        int i = 34;
        while (i) {
            chip8.run();
            i -= 2;
        }
    }
}