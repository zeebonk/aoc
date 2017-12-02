CC = g++
FLAGS = -std=c++17 -O2 -march=native -pedantic -Wall -Wextra
BUILD_DIR = build
TARGETS = $(addprefix $(BUILD_DIR)/, $(basename $(wildcard *.cpp)))

.PHONY: clean

all: $(TARGETS)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

run_%: $(BUILD_DIR)/%
	./$(BUILD_DIR)/$* < input/$*.txt

$(BUILD_DIR)/%: %.cpp $(BUILD_DIR)
	$(CC) $< -o ./$@ $(FLAGS)

clean:
	rm -rf $(BUILD_DIR)
