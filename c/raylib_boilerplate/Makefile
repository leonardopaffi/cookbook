BIN = bin/main.exe

CFLAGS = -std=c99

INCLUDE = include/

LIB = lib/

LDFLAGS = -lraylib -lopengl32 -lgdi32 -lwinmm

CC = tcc

SRC = src/main.c

all:
	$(CC) $(SRC) -o $(BIN) $(CFLAGS) -I $(INCLUDE) -L $(LIB) $(LDFLAGS)

run:
	./$(BIN)

clean:
	rm -f $(BIN)