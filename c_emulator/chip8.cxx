#include "chip8.hpp"
#include <fstream>
#include <iostream>
#include <stdlib.h>

Chip8::Chip8() {
    this->setup_ok = true;
    this->should_draw = false;
    this->should_quit = false;
    this->rom_size = 0;
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
    this->cls();
}

void Chip8::cls() {
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
    this->rom_size = rom.tellg();
    rom.seekg(0, std::ios::beg);
    char *buffer = new char[rom_size];
    rom.read(buffer, this->rom_size);
    for (int i = 0; i < rom_size; i++) {
        this->memory[this->PC + i] = buffer[i];
    }
    delete[] buffer;
    rom.close();
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
    this->should_draw = false;
}

void Chip8::run() {
    this->opcode = memory[this->PC];
    this->opcode <<= 8;
    this->opcode |= memory[this->PC + 1];
    switch (this->opcode & 0xf000) {
    case 0x0000:
        switch (this->opcode & 0x000f) {
        case 0x0000: // clear screen (0x00e0)
            this->cls();
            this->should_draw = true;
            this->PC += 2;
            break;
        case 0x000e: // ret from function (0x00ee)
            this->PC = this->stack[this->SP];
            this->SP--; // TODO: We will have to check the order
            break;
        default:
            std::cerr << "Unrecognized OPCode: " << std::hex << this->opcode
                      << std::dec << std::endl;
            break;
        }
        break;
    case 0x1000:
        this->PC = this->opcode & 0x0fff;
        break;
    case 0x2000:
        this->SP++;
        this->stack[this->SP] = this->PC;
        this->PC = this->opcode & 0x0fff;
        break;
    case 0x3000: {
        unsigned char register_index = this->opcode & 0x0f00;
        unsigned char value = this->opcode & 0x00ff;
        if (this->Vx[register_index] == value)
            this->PC += 2;
        this->PC += 2;
        break;
    }
    case 0x4000: {
        unsigned char register_index = this->opcode & 0x0f00;
        unsigned char value = this->opcode & 0x00ff;
        if (this->Vx[register_index] != value)
            this->PC += 2;
        this->PC += 2;
        break;
    }
    case 0x5000: {
        unsigned char register_x = this->opcode & 0x0f00;
        unsigned char register_y = this->opcode & 0x00f0;
        if (this->Vx[register_x] == this->Vx[register_y])
            this->PC += 2;
        this->PC += 2;
        break;
    }
    case 0x6000: {
        unsigned char register_index = this->opcode & 0x0f00;
        unsigned char value = this->opcode & 0x00ff;
        this->Vx[register_index] = value;
        this->PC += 2;
        break;
    }
    case 0x7000: {
        unsigned char register_index = this->opcode & 0x0f00;
        unsigned char value = this->opcode & 0x00ff;
        this->Vx[register_index] += value;
        this->PC += 2;
        break;
    }
    case 0x8000: {
        switch (this->opcode & 0x000f) {
        case 0: {
            unsigned char register_x = this->opcode & 0x0f00;
            unsigned char register_y = this->opcode & 0x00f0;
            this->Vx[register_x] = this->Vx[register_y];
            this->PC += 2;
            break;
        }
        case 1: {
            unsigned char register_x = this->opcode & 0x0f00;
            unsigned char register_y = this->opcode & 0x00f0;
            this->Vx[register_x] |= this->Vx[register_y];
            this->PC += 2;
            break;
        }
        case 2: {
            unsigned char register_x = this->opcode & 0x0f00;
            unsigned char register_y = this->opcode & 0x00f0;
            this->Vx[register_x] &= this->Vx[register_y];
            this->PC += 2;
            break;
        }
        case 3: {
            unsigned char register_x = this->opcode & 0x0f00;
            unsigned char register_y = this->opcode & 0x00f0;
            this->Vx[register_x] ^= this->Vx[register_y];
            this->PC += 2;
            break;
        }
        case 4: {
            unsigned short register_x = this->opcode & 0x0f00;
            unsigned short register_y = this->opcode & 0x00f0;
            unsigned short sum = this->Vx[register_x] + this->Vx[register_y];
            this->Vx[register_x] = sum & 0xff;
            if (sum > 0xff)
                this->Vx[0xf] = 1;
            else
                this->Vx[0xf] = 0;
            this->PC += 2;
            break;
        }
        case 5: {
            unsigned short register_x = this->opcode & 0x0f00;
            unsigned short register_y = this->opcode & 0x00f0;
            if (this->Vx[register_x] > this->Vx[register_y])
                this->Vx[0xf] = 1;
            else
                this->Vx[0xf] = 0;
            this->Vx[register_x] -= this->Vx[register_y];
            this->PC += 2;
            break;
        }
        case 6: {
            unsigned short register_x = this->opcode & 0x0f00;
            if (this->Vx[register_x] & 0x0001)
                this->Vx[0xf] = 1;
            else
                this->Vx[0xf] = 0;
            this->Vx[register_x] >>= 1;
            this->PC += 2;
            break;
        }
        case 7: {
            unsigned short register_x = this->opcode & 0x0f00;
            unsigned short register_y = this->opcode & 0x00f0;
            if (this->Vx[register_y] > this->Vx[register_x])
                this->Vx[0xf] = 1;
            else
                this->Vx[0xf] = 0;
            this->Vx[register_x] = this->Vx[register_y] - this->Vx[register_x];
            this->PC += 2;
            break;
        }
        case 0xe: {
            unsigned short register_x = this->opcode & 0x0f00;
            if (this->Vx[register_x] & 0x8000)
                this->Vx[0xf] = 1;
            else
                this->Vx[0xf] = 0;
            this->Vx[register_x] <<= 1;
            this->PC += 2;
            break;
        }
        default:
            std::cerr << "Unrecognized OPCode: " << std::hex << this->opcode
                      << std::dec << std::endl;
            break;
        }
        break;
    }
    case 0x9000: {
        unsigned char register_x = this->opcode & 0x0f00;
        unsigned char register_y = this->opcode & 0x00f0;
        if (this->Vx[register_x] != this->Vx[register_y])
            this->PC += 2;
        this->PC += 2;
        break;
    }
    case 0xa000:
        this->I = this->opcode & 0x0fff;
        this->PC += 2;
        break;
    case 0xb000:
        this->PC = this->Vx[0] + (this->opcode & 0x0fff);
        break;
    case 0xc000:{
        unsigned char register_index = this->opcode & 0x0f00;
        unsigned char value = this->opcode & 0x00ff;
        this->Vx[register_index] = (rand() % 0xff) & value;
        this->PC += 2;
        break;
    }
    case 0xd000:{
        unsigned short pos_x = this->Vx[this->opcode & 0x0f00];
        unsigned short pos_y = this->Vx[this->opcode & 0x00f0];
        unsigned short sprite_size = this->opcode & 0x000f;
        unsigned short pixel;
        this->Vx[0xf] = 0;
			for (int yline = 0; yline < sprite_size; yline++)
			{
				pixel = this->memory[this->I + yline];
				for(int xline = 0; xline < 8; xline++)
				{
					if((pixel & (0x80 >> xline)) != 0)
					{
						if(this->screen[(pos_x + xline + ((pos_y + yline) * 64))] == 1)
						{
							this->Vx[0xf] = 1;                                    
						}
						this->screen[pos_x + xline + ((pos_y + yline) * 64)] ^= 1;
					}
				}
			}
        this->PC += 2;
        this->should_draw = true;
        break;
    }

    default:
        std::cerr << "Unrecognized OPCode: " << std::hex << this->opcode
                  << std::dec << std::endl;
        this->PC += 2;
        break;
    }
}

Chip8::~Chip8() {
    SDL_DestroyRenderer(this->gRenderer);
    SDL_DestroyWindow(this->gWindow);
    SDL_Quit();
    IMG_Quit();
    this->gRenderer = nullptr;
    this->gWindow = nullptr;
}