#ifndef CHIP8_HPP
#define CHIP8_HPP

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <string>

class Chip8 {
  public:
    Chip8();
    void run();
    void loadRom(std::string romFilePath);
    bool setup_succesful() const { return this->setup_ok; }
    ~Chip8();

  private:
    const int screen_height = 32;
    const int screen_width = 64;
    const int scale = 10;
    const int screen_size = this->screen_height * this->screen_width;
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
    bool setup_ok;
    SDL_Renderer *gRenderer = nullptr;
    SDL_Window *gWindow = nullptr;

    void system_init();
    void graphics_init();
    void draw();
};
#endif