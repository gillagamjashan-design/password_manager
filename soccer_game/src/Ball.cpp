#include "Ball.h"
#include <algorithm>

Ball::Ball(float x, float y, float radius)
    : position(x, y), velocity(0.0f, 0.0f), radius(radius),
      friction(0.98f), maxSpeed(600.0f), lastKickerTeam(0) { // NEW: Initialize to 0 (no team)

    shape.setRadius(radius);
    shape.setFillColor(sf::Color::White);
    shape.setOrigin(radius, radius);
    shape.setPosition(position.x, position.y);
}

void Ball::update(float deltaTime) {
    // Apply velocity
    position += velocity * deltaTime;

    // Apply friction
    velocity *= friction;

    // Stop if moving very slowly
    if (velocity.length() < 1.0f) {
        velocity = Vec2(0.0f, 0.0f);
    }

    // Clamp velocity to max speed
    if (velocity.length() > maxSpeed) {
        velocity = velocity.normalized() * maxSpeed;
    }

    // Update shape position
    shape.setPosition(position.x, position.y);
}

void Ball::render(sf::RenderWindow& window) {
    window.draw(shape);
}

void Ball::kick(const Vec2& direction, float power) {
    velocity += direction.normalized() * power;

    // Clamp to max speed
    if (velocity.length() > maxSpeed) {
        velocity = velocity.normalized() * maxSpeed;
    }
}

void Ball::setPosition(const Vec2& pos) {
    position = pos;
    shape.setPosition(position.x, position.y);
}

// NEW: Directly set ball velocity (useful for stopping ball completely)
void Ball::setVelocity(const Vec2& vel) {
    velocity = vel;
}

Vec2 Ball::getPosition() const {
    return position;
}

Vec2 Ball::getVelocity() const {
    return velocity;
}

float Ball::getRadius() const {
    return radius;
}

void Ball::bounceX() {
    velocity.x = -velocity.x * 0.8f;
}

void Ball::bounceY() {
    velocity.y = -velocity.y * 0.8f;
}

// NEW: Set which team last kicked the ball
void Ball::setLastKicker(int teamId) {
    lastKickerTeam = teamId;
}

// NEW: Get which team last kicked the ball
int Ball::getLastKicker() const {
    return lastKickerTeam;
}
