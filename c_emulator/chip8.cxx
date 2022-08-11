#include "chip8.hpp"
#include <fstream>
#include <iostream>

Chip8::Chip8() {
    this->PC = 0x0200;
    this->opcode = 0x0000;
    this->SP = 0x00;
    this->delayTimer = 0x00;
    this->soundTimer = 0x00;
    this->I = 0x0000;
    for(int i=0;i<16;i++) {
        stack[i] = 0x0000;
    }
    for(int i=0;i<16;i++) {
        Vx[i] = 0x00;
    }
    for(int i=0;i<4096;i++) {
        memory[i] = 0x00;
    }
    for(int i=0;i<16;i++) {
        keyboard[i] = 0x00;
    }
    for(int i=0;i<(64*32);i++) {
        screen[i] = 0x00;
    }
}

void Chip8::loadRom(std::string romFilePath) {
    std::ifstream rom(romFilePath, std::ios::binary);
    rom.seekg(0,std::ios::end);
    auto rom_size = rom.tellg();
    rom.seekg(0, std::ios::beg);
    char * buffer = new char[rom_size];
    rom.read(buffer, rom_size);
    for(int i=0; i<rom_size; i++) {
        this->memory[this->PC + i] = buffer[i];
    }
    rom.close();
}

void Chip8::run() {
    this->opcode = memory[this->PC];
    this->opcode <<= 8;
    this->opcode |= memory[this->PC + 1];
    this->PC += 2;
    std::cout<<std::hex << this->opcode <<std::endl;
}