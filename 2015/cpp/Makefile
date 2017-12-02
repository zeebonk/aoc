CC = g++
FLAGS = -std=c++17 -O2 -march=native -Wall -Wextra
BUILD_DIR = build
TARGETS = $(addprefix $(BUILD_DIR)/day_, $(basename $(wildcard *.cpp)))

all: build_dir $(TARGETS)

build_dir:
	mkdir -p $(BUILD_DIR)

$(BUILD_DIR)/day_4_%: 4_%.cpp
	$(CC) $< -o ./$@ $(FLAGS) -lcrypto

$(BUILD_DIR)/day_%: %.cpp
	$(CC) $< -o ./$@ $(FLAGS)

clean:
	rm -rf $(BUILD_DIR)
