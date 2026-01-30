#include "Game.h"
#include <iostream>
#include <sstream>
#include <iomanip>

// NEW: Constructor now accepts game mode
Game::Game(GameMode mode)
    : window(sf::VideoMode(PITCH_WIDTH, PITCH_HEIGHT), "Soccer Game"),
      scoreTeam1(0), scoreTeam2(0), matchTime(0.0f),
      matchDuration(180.0f), gameRunning(true),
      currentMode(mode), playersPerTeam(static_cast<int>(mode)),
      outOfBoundsMessage(""), outOfBoundsMessageTime(0.0f) { // Initialize notification

    window.setFramerateLimit(60);

    // Initialize game objects with player count based on mode
    ball = std::make_unique<Ball>(PITCH_WIDTH / 2.0f, PITCH_HEIGHT / 2.0f, 10.0f);
    team1 = std::make_unique<Team>(Team::Side::LEFT, sf::Color::Blue, playersPerTeam);
    team2 = std::make_unique<Team>(Team::Side::RIGHT, sf::Color::Red, playersPerTeam);
    input = std::make_unique<Input>();

    // Load font (using SFML default font path)
    // Note: On Linux, you might need to install fonts or specify a path
    if (!font.loadFromFile("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")) {
        // Try alternative paths
        if (!font.loadFromFile("/System/Library/Fonts/Helvetica.ttc")) {
            if (!font.loadFromFile("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf")) {
                std::cerr << "Warning: Could not load font. UI text will not display.\n";
            }
        }
    }

    // NEW: Set pitch boundaries for all players
    float margin = 15.0f; // Player radius margin from edges
    for (auto& player : team1->getPlayers()) {
        player->setPitchBounds(margin, PITCH_WIDTH - margin, margin, PITCH_HEIGHT - margin);
    }
    for (auto& player : team2->getPlayers()) {
        player->setPitchBounds(margin, PITCH_WIDTH - margin, margin, PITCH_HEIGHT - margin);
    }

    resetPositions();
}

Game::~Game() = default;

void Game::run() {
    sf::Clock clock;
    float accumulator = 0.0f;

    while (window.isOpen() && gameRunning) {
        float frameTime = clock.restart().asSeconds();

        // Cap frame time to avoid spiral of death
        if (frameTime > 0.25f) {
            frameTime = 0.25f;
        }

        accumulator += frameTime;

        // Fixed time step updates
        while (accumulator >= FIXED_TIME_STEP) {
            processEvents();
            update(FIXED_TIME_STEP);
            accumulator -= FIXED_TIME_STEP;
        }

        render();
    }
}

void Game::processEvents() {
    sf::Event event;
    while (window.pollEvent(event)) {
        if (event.type == sf::Event::Closed) {
            window.close();
        }

        if (event.type == sf::Event::KeyPressed) {
            if (event.key.code == sf::Keyboard::Escape) {
                window.close();
            }
        }
    }
}

void Game::update(float deltaTime) {
    if (matchTime >= matchDuration) {
        gameRunning = false;
        return;
    }

    matchTime += deltaTime;

    // NEW: Update out-of-bounds message timer
    if (outOfBoundsMessageTime > 0.0f) {
        outOfBoundsMessageTime -= deltaTime;
        if (outOfBoundsMessageTime <= 0.0f) {
            outOfBoundsMessage = "";
        }
    }

    // Update input
    input->update();

    // Control player 1 from team 1
    if (!team1->getPlayers().empty()) {
        Player* controlledPlayer = team1->getPlayers()[0].get();
        Vec2 moveDir = input->getMovementDirection();

        if (moveDir.length() > 0.0f) {
            Vec2 targetPos = controlledPlayer->getPosition() + moveDir * 300.0f;
            controlledPlayer->moveTowards(targetPos, deltaTime);
        }

        // Kick ball
        if (input->isKickPressed()) {
            Vec2 kickDir = ball->getPosition() - controlledPlayer->getPosition();
            if (kickDir.length() < 0.01f) {
                kickDir = Vec2(1.0f, 0.0f); // Default direction
            }
            // NEW: Track that team 1 kicked the ball
            if (controlledPlayer->isNearBall(*ball, controlledPlayer->getRadius() + ball->getRadius() + 20.0f)) {
                ball->setLastKicker(1); // Team 1
            }
            controlledPlayer->kickBall(*ball, kickDir, 500.0f);
        }
    }

    // Update teams
    // NEW: Pass team ID (1 for team1, 2 for team2) to track ball kicks
    team1->update(deltaTime, *ball, false, 1); // Human controlled (AI disabled), Team ID = 1
    team2->update(deltaTime, *ball, true, 2);  // AI controlled, Team ID = 2

    // Update ball
    ball->update(deltaTime);

    // Handle collisions and game logic
    handleCollisions();
    checkGoals();
    checkOutOfBounds(); // NEW: Check if ball went out of bounds
}

void Game::render() {
    window.clear(sf::Color(34, 139, 34)); // Green pitch

    drawPitch();

    // Render game objects
    ball->render(window);
    team1->render(window);
    team2->render(window);

    drawUI();

    window.display();
}

void Game::handleCollisions() {
    Vec2 ballPos = ball->getPosition();
    float ballRadius = ball->getRadius();

    // Ball collision with pitch boundaries (excluding goals)
    if (ballPos.x - ballRadius < 0.0f) {
        ball->setPosition(Vec2(ballRadius, ballPos.y));
        ball->bounceX();
    }
    if (ballPos.x + ballRadius > PITCH_WIDTH) {
        ball->setPosition(Vec2(PITCH_WIDTH - ballRadius, ballPos.y));
        ball->bounceX();
    }

    // Top and bottom boundaries
    if (ballPos.y - ballRadius < 0.0f) {
        ball->setPosition(Vec2(ballPos.x, ballRadius));
        ball->bounceY();
    }
    if (ballPos.y + ballRadius > PITCH_HEIGHT) {
        ball->setPosition(Vec2(ballPos.x, PITCH_HEIGHT - ballRadius));
        ball->bounceY();
    }

    // Ball collision with players
    // NEW: Added teamId parameter to track which team touched the ball
    auto checkPlayerCollisions = [&](Team& team, int teamId) {
        for (auto& player : team.getPlayers()) {
            float dist = ballPos.distance(player->getPosition());
            float minDist = ballRadius + player->getRadius();

            if (dist < minDist) {
                // Simple collision response
                Vec2 pushDir = (ballPos - player->getPosition()).normalized();
                ball->setPosition(player->getPosition() + pushDir * minDist);

                // Transfer some velocity
                Vec2 impulse = pushDir * player->getVelocity().length() * 0.5f;
                ball->kick(impulse, 1.0f);

                // NEW: Track which team last touched the ball
                ball->setLastKicker(teamId);
            }
        }
    };

    checkPlayerCollisions(*team1, 1); // Team 1
    checkPlayerCollisions(*team2, 2); // Team 2
}

void Game::checkGoals() {
    Vec2 ballPos = ball->getPosition();
    float ballRadius = ball->getRadius();

    float goalTop = (PITCH_HEIGHT - GOAL_WIDTH) / 2.0f;
    float goalBottom = goalTop + GOAL_WIDTH;

    // Left goal (Team 2 scores)
    if (ballPos.x - ballRadius < 0.0f &&
        ballPos.y > goalTop && ballPos.y < goalBottom) {
        scoreTeam2++;
        resetPositions();
    }

    // Right goal (Team 1 scores)
    if (ballPos.x + ballRadius > PITCH_WIDTH &&
        ballPos.y > goalTop && ballPos.y < goalBottom) {
        scoreTeam1++;
        resetPositions();
    }
}

void Game::resetPositions() {
    ball->setPosition(Vec2(PITCH_WIDTH / 2.0f, PITCH_HEIGHT / 2.0f));
    ball->kick(Vec2(0.0f, 0.0f), 0.0f); // Stop the ball
    team1->resetPositions(PITCH_WIDTH, PITCH_HEIGHT);
    team2->resetPositions(PITCH_WIDTH, PITCH_HEIGHT);
}

void Game::drawPitch() {
    // Center circle
    sf::CircleShape centerCircle(80.0f);
    centerCircle.setFillColor(sf::Color::Transparent);
    centerCircle.setOutlineColor(sf::Color::White);
    centerCircle.setOutlineThickness(2.0f);
    centerCircle.setOrigin(80.0f, 80.0f);
    centerCircle.setPosition(PITCH_WIDTH / 2.0f, PITCH_HEIGHT / 2.0f);
    window.draw(centerCircle);

    // Center line
    sf::RectangleShape centerLine(sf::Vector2f(2.0f, PITCH_HEIGHT));
    centerLine.setFillColor(sf::Color::White);
    centerLine.setPosition(PITCH_WIDTH / 2.0f - 1.0f, 0.0f);
    window.draw(centerLine);

    // Goals
    float goalTop = (PITCH_HEIGHT - GOAL_WIDTH) / 2.0f;

    // Left goal
    sf::RectangleShape leftGoal(sf::Vector2f(GOAL_DEPTH, GOAL_WIDTH));
    leftGoal.setFillColor(sf::Color(100, 100, 100, 128));
    leftGoal.setOutlineColor(sf::Color::White);
    leftGoal.setOutlineThickness(2.0f);
    leftGoal.setPosition(-GOAL_DEPTH, goalTop);
    window.draw(leftGoal);

    // Right goal
    sf::RectangleShape rightGoal(sf::Vector2f(GOAL_DEPTH, GOAL_WIDTH));
    rightGoal.setFillColor(sf::Color(100, 100, 100, 128));
    rightGoal.setOutlineColor(sf::Color::White);
    rightGoal.setOutlineThickness(2.0f);
    rightGoal.setPosition(PITCH_WIDTH, goalTop);
    window.draw(rightGoal);
}

void Game::drawUI() {
    if (!font.getInfo().family.empty()) {
        // Score
        std::ostringstream scoreText;
        scoreText << "Blue " << scoreTeam1 << " - " << scoreTeam2 << " Red";

        sf::Text score(scoreText.str(), font, 24);
        score.setFillColor(sf::Color::White);
        score.setPosition(PITCH_WIDTH / 2.0f - 80.0f, 10.0f);
        window.draw(score);

        // Timer
        int minutes = static_cast<int>(matchTime) / 60;
        int seconds = static_cast<int>(matchTime) % 60;

        std::ostringstream timeText;
        timeText << std::setfill('0') << std::setw(2) << minutes << ":"
                 << std::setfill('0') << std::setw(2) << seconds;

        sf::Text timer(timeText.str(), font, 20);
        timer.setFillColor(sf::Color::White);
        timer.setPosition(PITCH_WIDTH / 2.0f - 30.0f, 40.0f);
        window.draw(timer);

        // NEW: Out-of-bounds notification message
        if (outOfBoundsMessageTime > 0.0f && !outOfBoundsMessage.empty()) {
            sf::Text oobText(outOfBoundsMessage, font, 28);
            oobText.setFillColor(sf::Color::Yellow);
            oobText.setStyle(sf::Text::Bold);

            // Center the text
            sf::FloatRect textBounds = oobText.getLocalBounds();
            oobText.setPosition(PITCH_WIDTH / 2.0f - textBounds.width / 2.0f, PITCH_HEIGHT / 2.0f - 100.0f);

            // Add a semi-transparent background
            sf::RectangleShape bgRect(sf::Vector2f(textBounds.width + 20.0f, textBounds.height + 20.0f));
            bgRect.setFillColor(sf::Color(0, 0, 0, 180));
            bgRect.setPosition(PITCH_WIDTH / 2.0f - textBounds.width / 2.0f - 10.0f, PITCH_HEIGHT / 2.0f - 110.0f);

            window.draw(bgRect);
            window.draw(oobText);
        }

        // Game over message
        if (matchTime >= matchDuration) {
            sf::Text gameOver("GAME OVER!", font, 48);
            gameOver.setFillColor(sf::Color::Yellow);
            gameOver.setPosition(PITCH_WIDTH / 2.0f - 150.0f, PITCH_HEIGHT / 2.0f - 50.0f);
            window.draw(gameOver);

            std::string winner = (scoreTeam1 > scoreTeam2) ? "Blue Wins!" :
                                 (scoreTeam2 > scoreTeam1) ? "Red Wins!" : "Draw!";
            sf::Text winnerText(winner, font, 32);
            winnerText.setFillColor(sf::Color::White);
            winnerText.setPosition(PITCH_WIDTH / 2.0f - 80.0f, PITCH_HEIGHT / 2.0f + 10.0f);
            window.draw(winnerText);
        }

        // NEW: Display game mode in corner
        std::string modeText = "Mode: ";
        switch(currentMode) {
            case GameMode::ONE_V_ONE:
                modeText += "1v1";
                break;
            case GameMode::TWO_V_TWO:
                modeText += "2v2";
                break;
            case GameMode::THREE_V_THREE:
                modeText += "3v3";
                break;
        }
        sf::Text modeDisplay(modeText, font, 16);
        modeDisplay.setFillColor(sf::Color(200, 200, 200));
        modeDisplay.setPosition(10.0f, PITCH_HEIGHT - 25.0f);
        window.draw(modeDisplay);
    }
}

// NEW: Check if ball went out of bounds (excluding goals)
void Game::checkOutOfBounds() {
    Vec2 ballPos = ball->getPosition();
    float ballRadius = ball->getRadius();

    float goalTop = (PITCH_HEIGHT - GOAL_WIDTH) / 2.0f;
    float goalBottom = goalTop + GOAL_WIDTH;

    bool isInGoalZone = (ballPos.y >= goalTop && ballPos.y <= goalBottom);

    // Check left side (not in goal area)
    if (ballPos.x - ballRadius < 0.0f && !isInGoalZone) {
        int lastKicker = ball->getLastKicker();
        // If team 1 kicked it out on the left, team 2 gets possession
        // If team 2 kicked it out, team 2 gets possession (defensive end)
        int possessionTeam = (lastKicker == 1) ? 2 : 1;
        handleThrowIn(false, 50.0f, possessionTeam);
        return;
    }

    // Check right side (not in goal area)
    if (ballPos.x + ballRadius > PITCH_WIDTH && !isInGoalZone) {
        int lastKicker = ball->getLastKicker();
        // If team 2 kicked it out on the right, team 1 gets possession
        // If team 1 kicked it out, team 1 gets possession (defensive end)
        int possessionTeam = (lastKicker == 2) ? 1 : 2;
        handleThrowIn(false, PITCH_WIDTH - 50.0f, possessionTeam);
        return;
    }

    // Check top boundary
    if (ballPos.y - ballRadius < 0.0f) {
        int lastKicker = ball->getLastKicker();
        // Team that didn't kick it out gets possession
        int possessionTeam = (lastKicker == 1) ? 2 : 1;
        handleThrowIn(true, ballPos.x, possessionTeam);
        return;
    }

    // Check bottom boundary
    if (ballPos.y + ballRadius > PITCH_HEIGHT) {
        int lastKicker = ball->getLastKicker();
        // Team that didn't kick it out gets possession
        int possessionTeam = (lastKicker == 1) ? 2 : 1;
        handleThrowIn(false, ballPos.x, possessionTeam);
        return;
    }
}

// NEW: Handle throw-in / ball reset after out of bounds
void Game::handleThrowIn(bool fromTop, float xPos, int possessionTeam) {
    // Reset ball position near the boundary
    float yPos;
    if (fromTop) {
        yPos = 30.0f; // Near top boundary
    } else {
        yPos = PITCH_HEIGHT - 30.0f; // Near bottom boundary
    }

    // Clamp xPos to valid range
    if (xPos < 50.0f) xPos = 50.0f;
    if (xPos > PITCH_WIDTH - 50.0f) xPos = PITCH_WIDTH - 50.0f;

    // Reset ball position and stop it
    ball->setPosition(Vec2(xPos, yPos));
    ball->kick(Vec2(0.0f, 0.0f), 0.0f); // Stop the ball

    // Move a player from the possession team near the ball
    Team* team = (possessionTeam == 1) ? team1.get() : team2.get();
    if (!team->getPlayers().empty()) {
        Player* nearestPlayer = team->getPlayers()[0].get();
        Vec2 playerPos(xPos + 30.0f, yPos);
        nearestPlayer->setPosition(playerPos);
        nearestPlayer->setVelocity(Vec2(0.0f, 0.0f));
    }

    // NEW: Set notification message
    std::string teamName = (possessionTeam == 1) ? "BLUE" : "RED";
    outOfBoundsMessage = "OUT OF BOUNDS! " + teamName + " team gets possession";
    outOfBoundsMessageTime = MESSAGE_DISPLAY_DURATION;

    std::cout << "Out of bounds! Team " << possessionTeam << " (" << teamName << ") gets possession.\n";
}
