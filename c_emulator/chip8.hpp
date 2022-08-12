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
    bool render_ready() const { return this->should_draw;}
    bool quit() const {return this->should_quit;}
    void draw();
    void capture_keys();
    ~Chip8();

  private:
    const int screen_height = 32;
    const int screen_width = 64;
    const int scale = 12;
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
    unsigned short rom_size;
    bool setup_ok;
    bool should_draw;
    bool should_quit;
    SDL_Renderer *gRenderer = nullptr;
    SDL_Window *gWindow = nullptr;
    SDL_Event key_event;
unsigned char chip8_fontset[80] =
  {
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
  };
  unsigned char keymap[16] = {
    SDLK_x,
    SDLK_1,
    SDLK_2,
    SDLK_3,
    SDLK_q,
    SDLK_w,
    SDLK_e,
    SDLK_a,
    SDLK_s,
    SDLK_d,
    SDLK_z,
    SDLK_c,
    SDLK_4,
    SDLK_r,
    SDLK_f,
    SDLK_v,
};
    void system_init();
    void graphics_init();
    void cls();
    void load_font();
};
#endif