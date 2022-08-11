#ifndef CHIP8_HPP
#define CHIP8_HPP

#include <string>

class Chip8{
public:
    Chip8();
    void run();
    void loadRom(std::string romFilePath);
    ~Chip8(){};
private:
    unsigned short I;
    unsigned short PC;
    unsigned char SP;
    unsigned char delayTimer;
    unsigned char soundTimer;
    unsigned short opcode;
    unsigned short stack[16];
    unsigned char Vx[16];
    unsigned char memory[4096];
    unsigned char keyboard[16];
    unsigned char screen[64 * 32];
};
#endif