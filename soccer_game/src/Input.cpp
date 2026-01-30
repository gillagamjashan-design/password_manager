#include "Input.h"

Input::Input() : movementDirection(0.0f, 0.0f), kickPressed(false), kickWasPressed(false) {
}

void Input::update() {
    // Reset movement direction
    movementDirection = Vec2(0.0f, 0.0f);

    // Check arrow keys
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Left)) {
        movementDirection.x -= 1.0f;
    }
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Right)) {
        movementDirection.x += 1.0f;
    }
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) {
        movementDirection.y -= 1.0f;
    }
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) {
        movementDirection.y += 1.0f;
    }

    // Normalize diagonal movement
    if (movementDirection.length() > 0.0f) {
        movementDirection.normalize();
    }

    // Check kick button (Space) - single press detection
    bool spacePressed = sf::Keyboard::isKeyPressed(sf::Keyboard::Space);
    kickPressed = spacePressed && !kickWasPressed;
    kickWasPressed = spacePressed;
}

Vec2 Input::getMovementDirection() const {
    return movementDirection;
}

bool Input::isKickPressed() const {
    return kickPressed;
}
