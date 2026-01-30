#include "Team.h"
#include <algorithm>
#include <limits>

Team::Team(Side side, sf::Color color, int numPlayers)
    : side(side), teamColor(color) {

    // Create players
    for (int i = 0; i < numPlayers; ++i) {
        float x = (side == Side::LEFT) ? 200.0f : 700.0f;
        float y = 150.0f + i * 150.0f;
        players.push_back(std::make_unique<Player>(x, y, 15.0f, color));
    }
}

// NEW: Added teamId parameter to track which team is kicking
void Team::update(float deltaTime, Ball& ball, bool isAI, int teamId) {
    if (isAI) {
        updateAI(deltaTime, ball, teamId);
    }

    // Update all players
    for (auto& player : players) {
        player->update(deltaTime);
    }
}

void Team::render(sf::RenderWindow& window) {
    for (auto& player : players) {
        player->render(window);
    }
}

void Team::resetPositions(float pitchWidth, float pitchHeight) {
    float xBase = (side == Side::LEFT) ? pitchWidth * 0.25f : pitchWidth * 0.75f;
    float ySpacing = pitchHeight / (players.size() + 1);

    for (size_t i = 0; i < players.size(); ++i) {
        float y = ySpacing * (i + 1);
        players[i]->setPosition(Vec2(xBase, y));
        players[i]->setVelocity(Vec2(0.0f, 0.0f));
    }
}

Player* Team::getClosestPlayerToBall(const Ball& ball) {
    Player* closest = nullptr;
    float minDistance = std::numeric_limits<float>::max();

    for (auto& player : players) {
        float dist = player->getPosition().distance(ball.getPosition());
        if (dist < minDistance) {
            minDistance = dist;
            closest = player.get();
        }
    }

    return closest;
}

std::vector<std::unique_ptr<Player>>& Team::getPlayers() {
    return players;
}

Team::Side Team::getSide() const {
    return side;
}

// NEW: Added teamId parameter to track which team is kicking
void Team::updateAI(float deltaTime, Ball& ball, int teamId) {
    // Simple AI: closest player chases the ball
    Player* closest = getClosestPlayerToBall(ball);

    if (closest) {
        Vec2 ballPos = ball.getPosition();
        closest->moveTowards(ballPos, deltaTime);

        // Try to kick towards opponent's goal
        if (closest->isNearBall(ball, closest->getRadius() + ball.getRadius() + 20.0f)) {
            Vec2 goalDirection;
            if (side == Side::LEFT) {
                goalDirection = Vec2(1.0f, 0.0f); // Kick right
            } else {
                goalDirection = Vec2(-1.0f, 0.0f); // Kick left
            }

            // Add some randomness
            goalDirection.y = (std::rand() % 100 - 50) / 100.0f;

            // NEW: Track that this team kicked the ball
            ball.setLastKicker(teamId);

            closest->kickBall(ball, goalDirection, 400.0f);
        }
    }

    // Other players return to defensive positions
    float xDefensive = (side == Side::LEFT) ? 200.0f : 700.0f;
    for (auto& player : players) {
        if (player.get() != closest) {
            Vec2 defendPos(xDefensive, player->getPosition().y);
            Vec2 direction = defendPos - player->getPosition();
            if (direction.length() > 50.0f) {
                player->moveTowards(defendPos, deltaTime * 0.5f);
            }
        }
    }
}
