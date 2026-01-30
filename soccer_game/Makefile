# Soccer Game Makefile with SFML support

# Compiler
CXX := g++

# Try to detect SFML using pkg-config first
PKG_CONFIG_SFML := $(shell pkg-config --exists sfml-all 2>/dev/null && echo "yes")

ifeq ($(PKG_CONFIG_SFML),yes)
    # Use pkg-config if available
    SFML_CFLAGS := $(shell pkg-config --cflags sfml-all)
    SFML_LIBS := $(shell pkg-config --libs sfml-all)
else
    # Manual fallback - standard Linux paths after apt-get install libsfml-dev
    # These are the paths where libsfml-dev installs files
    SFML_CFLAGS :=
    SFML_LIBS := -lsfml-graphics -lsfml-window -lsfml-system

    # macOS Homebrew paths (uncomment if on macOS)
    # SFML_CFLAGS := -I/usr/local/include -I/opt/homebrew/include
    # SFML_LIBS := -L/usr/local/lib -L/opt/homebrew/lib -lsfml-graphics -lsfml-window -lsfml-system
endif

# Compiler flags
CXXFLAGS := -std=c++17 -Wall -Wextra -Iinclude $(SFML_CFLAGS)
LDFLAGS := $(SFML_LIBS)

# Display help information
help:
	@echo "Soccer Game - Available Make Targets:"
	@echo ""
	@echo "  make all          - Build the soccer game executable"
	@echo "  make run          - Build and run the soccer game"
	@echo "  make clean        - Remove compiled executables"
	@echo "  make test-sfml    - Build and run SFML installation test"
	@echo "  make show-config  - Display detected SFML paths and flags"
	@echo "  make help         - Display this help message"
	@echo ""
	@echo "Quick start:"
	@echo "  1. Install SFML: sudo apt-get install libsfml-dev"
	@echo "  2. Test SFML:    make test-sfml"
	@echo "  3. Play game:    make run"

# Build the game
all:
	$(CXX) $(CXXFLAGS) src/*.cpp -o soccer_game $(LDFLAGS)

# Build and run the game
run: all
	./soccer_game

# Build and run SFML test
test-sfml:
	@echo "Compiling SFML test..."
	$(CXX) -std=c++17 sfml_test.cpp -o sfml_test $(LDFLAGS)
	@echo "Running SFML test..."
	./sfml_test

# Remove executables
clean:
	rm -f soccer_game sfml_test

# Display detected SFML configuration (useful for debugging)
show-config:
	@echo "PKG_CONFIG_SFML: $(PKG_CONFIG_SFML)"
	@echo "SFML_CFLAGS: $(SFML_CFLAGS)"
	@echo "SFML_LIBS: $(SFML_LIBS)"
	@echo ""
	@echo "Full compile command:"
	@echo "$(CXX) $(CXXFLAGS) src/*.cpp -o soccer_game $(LDFLAGS)"

.PHONY: all run clean test-sfml show-config help
