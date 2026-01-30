#include "Player.h"
#include "Ball.h"
#include <algorithm>

Player::Player(float x, float y, float radius, sf::Color color)
    : position(x, y), velocity(0.0f, 0.0f), radius(radius),
      maxSpeed(200.0f), acceleration(800.0f), color(color),
      pitchMinX(0.0f), pitchMaxX(10000.0f), pitchMinY(0.0f), pitchMaxY(10000.0f),
      hasBounds(false) { // NEW: Initialize with no bounds by default

    shape.setRadius(radius);
    shape.setFillColor(color);
    shape.setOrigin(radius, radius);
    shape.setPosition(position.x, position.y);
}

void Player::update(float deltaTime) {
    // Apply velocity
    position += velocity * deltaTime;

    // NEW: Clamp position to pitch boundaries (if bounds are set)
    if (hasBounds) {
        if (position.x < pitchMinX) {
            position.x = pitchMinX;
            velocity.x = 0.0f; // Stop horizontal movement when hitting boundary
        }
        if (position.x > pitchMaxX) {
            position.x = pitchMaxX;
            velocity.x = 0.0f;
        }
        if (position.y < pitchMinY) {
            position.y = pitchMinY;
            velocity.y = 0.0f; // Stop vertical movement when hitting boundary
        }
        if (position.y > pitchMaxY) {
            position.y = pitchMaxY;
            velocity.y = 0.0f;
        }
    }

    // Apply friction/deceleration
    velocity *= 0.85f;

    // Stop if moving very slowly
    if (velocity.length() < 5.0f) {
        velocity = Vec2(0.0f, 0.0f);
    }

    // Update shape position
    shape.setPosition(position.x, position.y);
}

void Player::render(sf::RenderWindow& window) {
    window.draw(shape);
}

void Player::moveTowards(const Vec2& target, float deltaTime) {
    Vec2 direction = target - position;
    float distance = direction.length();

    if (distance > 5.0f) {
        direction.normalize();
        velocity += direction * acceleration * deltaTime;

        // Clamp velocity
        if (velocity.length() > maxSpeed) {
            velocity = velocity.normalized() * maxSpeed;
        }
    }
}

void Player::setVelocity(const Vec2& vel) {
    velocity = vel;

    // Clamp velocity
    if (velocity.length() > maxSpeed) {
        velocity = velocity.normalized() * maxSpeed;
    }
}

void Player::setPosition(const Vec2& pos) {
    position = pos;
    shape.setPosition(position.x, position.y);
}

Vec2 Player::getPosition() const {
    return position;
}

Vec2 Player::getVelocity() const {
    return velocity;
}

float Player::getRadius() const {
    return radius;
}

bool Player::isNearBall(const Ball& ball, float threshold) const {
    return position.distance(ball.getPosition()) < threshold;
}

void Player::kickBall(Ball& ball, const Vec2& direction, float power) {
    if (isNearBall(ball, radius + ball.getRadius() + 20.0f)) {
        ball.kick(direction, power);
    }
}

// NEW: Set the pitch boundaries for this player
void Player::setPitchBounds(float minX, float maxX, float minY, float maxY) {
    pitchMinX = minX;
    pitchMaxX = maxX;
    pitchMinY = minY;
    pitchMaxY = maxY;
    hasBounds = true;
}
