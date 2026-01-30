#ifndef BALL_H
#define BALL_H

#include <SFML/Graphics.hpp>
#include "Vec2.h"

class Ball {
public:
    Ball(float x, float y, float radius);

    void update(float deltaTime);
    void render(sf::RenderWindow& window);

    void kick(const Vec2& direction, float power);
    void setPosition(const Vec2& pos);
    void setVelocity(const Vec2& vel); // NEW: Directly set velocity (for stopping ball)
    Vec2 getPosition() const;
    Vec2 getVelocity() const;
    float getRadius() const;

    void bounceX();
    void bounceY();

    // NEW: Track which team last kicked the ball (for out-of-bounds logic)
    // teamId: 0 = no team, 1 = team1, 2 = team2
    void setLastKicker(int teamId);
    int getLastKicker() const;

private:
    Vec2 position;
    Vec2 velocity;
    float radius;
    float friction;
    float maxSpeed;

    sf::CircleShape shape;

    // NEW: Track last team to kick the ball
    int lastKickerTeam; // 0 = none, 1 = team1, 2 = team2
};

#endif // BALL_H
