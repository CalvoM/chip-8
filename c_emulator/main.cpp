#include <iostream>
#include "chip8.hpp"

int main(int argc, char **argv) {
    Chip8 chip8;
    chip8.loadRom("../roms/chip-8/MAZE");
    int i = 34;
    while(i) {
        chip8.run();
        i -=2;
    }
}