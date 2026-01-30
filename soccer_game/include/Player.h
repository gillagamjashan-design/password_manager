#ifndef PLAYER_H
#define PLAYER_H

#include <SFML/Graphics.hpp>
#include "Vec2.h"

class Ball;

class Player {
public:
    Player(float x, float y, float radius, sf::Color color);

    void update(float deltaTime);
    void render(sf::RenderWindow& window);

    void moveTowards(const Vec2& target, float deltaTime);
    void setVelocity(const Vec2& vel);
    void setPosition(const Vec2& pos);

    Vec2 getPosition() const;
    Vec2 getVelocity() const;
    float getRadius() const;

    bool isNearBall(const Ball& ball, float threshold) const;
    void kickBall(Ball& ball, const Vec2& direction, float power);

    // NEW: Set pitch boundaries for player movement
    void setPitchBounds(float minX, float maxX, float minY, float maxY);

private:
    Vec2 position;
    Vec2 velocity;
    float radius;
    float maxSpeed;
    float acceleration;

    sf::CircleShape shape;
    sf::Color color;

    // NEW: Pitch boundaries to keep player on field
    float pitchMinX, pitchMaxX, pitchMinY, pitchMaxY;
    bool hasBounds; // Track if bounds have been set
};

#endif // PLAYER_H
