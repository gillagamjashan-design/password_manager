#ifndef TEAM_H
#define TEAM_H

#include <vector>
#include <memory>
#include <SFML/Graphics.hpp>
#include "Player.h"
#include "Ball.h"

class Team {
public:
    enum class Side {
        LEFT,
        RIGHT
    };

    Team(Side side, sf::Color color, int numPlayers = 3);

    // NEW: Changed Ball to non-const reference, added teamId parameter
    void update(float deltaTime, Ball& ball, bool isAI, int teamId);
    void render(sf::RenderWindow& window);

    void resetPositions(float pitchWidth, float pitchHeight);
    Player* getClosestPlayerToBall(const Ball& ball);
    std::vector<std::unique_ptr<Player>>& getPlayers();

    Side getSide() const;

private:
    // NEW: Changed Ball to non-const reference, added teamId parameter
    void updateAI(float deltaTime, Ball& ball, int teamId);

    std::vector<std::unique_ptr<Player>> players;
    Side side;
    sf::Color teamColor;
};

#endif // TEAM_H
