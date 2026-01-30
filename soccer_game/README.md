# Soccer Game

A simple 2D top-down soccer game written in modern C++ using SFML.

## Features

- Two teams with AI-controlled opponents
- Basic physics simulation for ball movement
- Player controls with passing and shooting
- Goal detection and scoreboard
- Match timer

## Requirements

- C++17 compatible compiler (g++ recommended)
- SFML 2.5 or newer
- Make

## Installation

### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install build-essential libsfml-dev
```

### macOS (using Homebrew)

```bash
brew install sfml
```

## Building

```bash
make
```

## Running

```bash
make run
```

Or directly:

```bash
./soccer_game
```

## Controls

- **Arrow Keys**: Move your player (Team 1, Player 1)
- **Space**: Kick/Shoot the ball
- **ESC**: Exit game

## Cleaning

```bash
make clean
```

## Project Structure

- `src/` - Source files (.cpp)
- `include/` - Header files (.h)
- `obj/` - Compiled object files (generated)
