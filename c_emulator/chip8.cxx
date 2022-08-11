#include "chip8.hpp"
#include <fstream>
#include <iostream>
#include <stdlib.h>

Chip8::Chip8() {
    this->setup_ok = true;
    this->graphics_init();
    this->system_init();
}

void Chip8::system_init() {
    this->PC = 0x0200;
    this->opcode = 0x0000;
    this->SP = 0x00;
    this->delayTimer = 0x00;
    this->soundTimer = 0x00;
    this->I = 0x0000;
    for (int i = 0; i < 16; i++) {
        stack[i] = 0x0000;
    }
    for (int i = 0; i < 16; i++) {
        Vx[i] = 0x00;
    }
    for (int i = 0; i < 4096; i++) {
        memory[i] = 0x00;
    }
    for (int i = 0; i < 16; i++) {
        keyboard[i] = 0x00;
    }
    for (int i = 0; i < this->screen_size; i++) {
        screen[i] = 0x00;
    }
}

void Chip8::graphics_init() {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        std::cerr << "Failed to initialize the graphics\n";
        std::cerr << SDL_GetError << std::endl;
        this->setup_ok = false;
    } else {
        SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "1");
        this->gWindow = SDL_CreateWindow(
            "Gaming Emulator", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
            this->screen_width * this->scale, this->screen_height * this->scale,
            SDL_WINDOW_SHOWN);
        if (this->gWindow == nullptr) {
            std::cerr << "Failed to initialize the graphics\n";
            std::cerr << SDL_GetError << std::endl;
            this->setup_ok = false;
        } else {
            this->gRenderer =
                SDL_CreateRenderer(this->gWindow, -1, SDL_RENDERER_ACCELERATED);
            if (this->gRenderer == nullptr) {
                std::cerr << "Failed to initialize the graphics\n";
                this->setup_ok = false;
            } else {
                if (SDL_RenderSetScale(this->gRenderer, this->scale,
                                       this->scale) < 0) {
                    std::cerr << SDL_GetError << std::endl;
                    this->setup_ok = false;
                }
                SDL_SetRenderDrawColor(this->gRenderer, 0xff, 0xff, 0xff, 0xff);
                int imgFlags = IMG_INIT_PNG;
                if (!(IMG_Init(imgFlags) & imgFlags)) {
                    std::cerr << "Failed to initialize the graphics\n";
                    this->setup_ok = false;
                }
            }
        }
    }
}

void Chip8::loadRom(std::string romFilePath) {
    std::ifstream rom(romFilePath, std::ios::binary);
    rom.seekg(0, std::ios::end);
    auto rom_size = rom.tellg();
    rom.seekg(0, std::ios::beg);
    char *buffer = new char[rom_size];
    rom.read(buffer, rom_size);
    for (int i = 0; i < rom_size; i++) {
        this->memory[this->PC + i] = buffer[i];
    }
    delete[] buffer;
    rom.close();
}

void Chip8::run() {
    this->opcode = memory[this->PC];
    this->opcode <<= 8;
    this->opcode |= memory[this->PC + 1];
    this->PC += 2;
}

void Chip8::draw() {
    SDL_SetRenderDrawColor(this->gRenderer, 0x00, 0x00, 0x00, 0xff);
    SDL_RenderClear(this->gRenderer);
    SDL_SetRenderDrawColor(this->gRenderer, 0xff, 0xff, 0xff, 0xff);
    int rowNum;
    for (int y = 0; y < this->screen_height; y++) {
        for (int x = 0; x < this->screen_width; x++) {
            rowNum = y * this->screen_width;
            if (screen[x + rowNum] != 0) {
                if (SDL_RenderDrawPoint(gRenderer, x, y) < 0) {
                    std::cerr << SDL_GetError << std::endl;
                }
            }
        }
    }
    SDL_RenderPresent(this->gRenderer);
}

Chip8::~Chip8() {
    SDL_DestroyRenderer(this->gRenderer);
    SDL_DestroyWindow(this->gWindow);
    SDL_Quit();
    IMG_Quit();
    this->gRenderer = nullptr;
    this->gWindow = nullptr;
}