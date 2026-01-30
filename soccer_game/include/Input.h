#ifndef INPUT_H
#define INPUT_H

#include <SFML/Graphics.hpp>
#include "Vec2.h"

class Input {
public:
    Input();

    void update();
    Vec2 getMovementDirection() const;
    bool isKickPressed() const;

private:
    Vec2 movementDirection;
    bool kickPressed;
    bool kickWasPressed; // For single-press detection
};

#endif // INPUT_H
