BINFILE:=chip8
COMPILER_FLAGS:=-w -g
LD_FLAGS:=-lSDL2 -lSDL2_image -lSDL2_mixer
all: build
	./$(BINFILE)
build:
	g++ $(COMPILER_FLAGS) *.cpp *.cxx -I . $(LD_FLAGS) -o $(BINFILE) 
memory: build
	valgrind --leak-check=full \
			--show-leak-kinds=all \
			--track-origins=yes \
			./chip8
style:
	find . -iname "*.h" -o -iname "*.cpp" -o -iname "*.cxx" | xargs clang-format -i
