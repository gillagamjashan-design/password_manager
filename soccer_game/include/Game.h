#ifndef GAME_H
#define GAME_H

#include <SFML/Graphics.hpp>
#include <memory>
#include <string>
#include "Ball.h"
#include "Team.h"
#include "Input.h"

// NEW: Game mode enum for different player counts
enum class GameMode {
    ONE_V_ONE = 1,
    TWO_V_TWO = 2,
    THREE_V_THREE = 3
};

class Game {
public:
    // NEW: Constructor now accepts game mode (defaults to 3v3)
    Game(GameMode mode = GameMode::THREE_V_THREE);
    ~Game();

    void run();

private:
    void processEvents();
    void update(float deltaTime);
    void render();

    void handleCollisions();
    void checkGoals();
    void resetPositions();
    void drawPitch();
    void drawUI();

    // NEW: Handle ball going out of bounds
    void checkOutOfBounds();
    void handleThrowIn(bool fromTop, float xPos, int possessionTeam, const std::string& side = "");

    // Window
    sf::RenderWindow window;

    // Game objects
    std::unique_ptr<Ball> ball;
    std::unique_ptr<Team> team1;
    std::unique_ptr<Team> team2;
    std::unique_ptr<Input> input;

    // Game state
    int scoreTeam1;
    int scoreTeam2;
    float matchTime;
    float matchDuration; // in seconds
    bool gameRunning;

    // NEW: Game mode (1v1, 2v2, or 3v3)
    GameMode currentMode;
    int playersPerTeam; // Number of players per team based on mode

    // NEW: Out-of-bounds notification
    std::string outOfBoundsMessage;
    float outOfBoundsMessageTime;
    static constexpr float MESSAGE_DISPLAY_DURATION = 2.0f;

    // Pitch dimensions
    static constexpr float PITCH_WIDTH = 900.0f;
    static constexpr float PITCH_HEIGHT = 600.0f;
    static constexpr float GOAL_WIDTH = 150.0f;
    static constexpr float GOAL_DEPTH = 20.0f;

    // Fixed time step
    static constexpr float FIXED_TIME_STEP = 1.0f / 60.0f;

    // Font for UI
    sf::Font font;
};

#endif // GAME_H
