#include "Game.h"
#include <iostream>
#include <cstdlib>
#include <ctime>

// Display main menu and get user's game mode selection
GameMode showMenu() {
    std::cout << "\n";
    std::cout << "========================================\n";
    std::cout << "        SOCCER GAME - MAIN MENU         \n";
    std::cout << "========================================\n";
    std::cout << "\n";
    std::cout << "Select Game Mode:\n";
    std::cout << "  [1] 1v1 - One player per team\n";
    std::cout << "  [2] 2v2 - Two players per team\n";
    std::cout << "  [3] 3v3 - Three players per team\n";
    std::cout << "\n";
    std::cout << "Enter your choice (1-3): ";

    int choice;
    std::cin >> choice;

    // Validate input
    while (std::cin.fail() || choice < 1 || choice > 3) {
        std::cin.clear();
        std::cin.ignore(10000, '\n');
        std::cout << "Invalid choice! Please enter 1, 2, or 3: ";
        std::cin >> choice;
    }

    std::cout << "\n";

    // Convert choice to GameMode
    GameMode mode;
    switch(choice) {
        case 1:
            mode = GameMode::ONE_V_ONE;
            std::cout << "Starting 1v1 mode...\n";
            break;
        case 2:
            mode = GameMode::TWO_V_TWO;
            std::cout << "Starting 2v2 mode...\n";
            break;
        case 3:
        default:
            mode = GameMode::THREE_V_THREE;
            std::cout << "Starting 3v3 mode...\n";
            break;
    }

    std::cout << "\n";
    std::cout << "CONTROLS:\n";
    std::cout << "  Arrow Keys - Move your player (Blue team)\n";
    std::cout << "  Space      - Kick/shoot the ball\n";
    std::cout << "  ESC        - Exit game\n";
    std::cout << "\n";
    std::cout << "FEATURES:\n";
    std::cout << "  - Players cannot move outside the pitch\n";
    std::cout << "  - Ball goes out of bounds = other team gets possession\n";
    std::cout << "  - Match duration: 3 minutes\n";
    std::cout << "\n";
    std::cout << "Press Enter to start the game...";
    std::cin.ignore();
    std::cin.get();
    std::cout << "\n";

    return mode;
}

int main() {
    // Seed random number generator for AI
    std::srand(static_cast<unsigned>(std::time(nullptr)));

    try {
        // Show menu and get user's choice
        GameMode selectedMode = showMenu();

        // Start the game with selected mode
        Game game(selectedMode);
        game.run();

        std::cout << "\nThanks for playing!\n";

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
