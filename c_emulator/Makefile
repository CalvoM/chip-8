BINFILE:=chip8
all: build
	./$(BINFILE)
build:
	g++ -g *.cpp *.cxx -I . -o $(BINFILE)
memory: build
	valgrind --leak-check=full \
			--show-leak-kinds=all \
			--track-origins=yes \
			./chip8
