#ifndef VEC2_H
#define VEC2_H

#include <cmath>

class Vec2 {
public:
    float x, y;

    Vec2() : x(0.0f), y(0.0f) {}
    Vec2(float x, float y) : x(x), y(y) {}

    // Vector addition
    Vec2 operator+(const Vec2& other) const {
        return Vec2(x + other.x, y + other.y);
    }

    Vec2& operator+=(const Vec2& other) {
        x += other.x;
        y += other.y;
        return *this;
    }

    // Vector subtraction
    Vec2 operator-(const Vec2& other) const {
        return Vec2(x - other.x, y - other.y);
    }

    Vec2& operator-=(const Vec2& other) {
        x -= other.x;
        y -= other.y;
        return *this;
    }

    // Scalar multiplication
    Vec2 operator*(float scalar) const {
        return Vec2(x * scalar, y * scalar);
    }

    Vec2& operator*=(float scalar) {
        x *= scalar;
        y *= scalar;
        return *this;
    }

    // Scalar division
    Vec2 operator/(float scalar) const {
        return Vec2(x / scalar, y / scalar);
    }

    // Length (magnitude)
    float length() const {
        return std::sqrt(x * x + y * y);
    }

    // Squared length (faster, avoids sqrt)
    float lengthSquared() const {
        return x * x + y * y;
    }

    // Normalize (return unit vector)
    Vec2 normalized() const {
        float len = length();
        if (len > 0.0f) {
            return Vec2(x / len, y / len);
        }
        return Vec2(0.0f, 0.0f);
    }

    // Normalize in place
    void normalize() {
        float len = length();
        if (len > 0.0f) {
            x /= len;
            y /= len;
        }
    }

    // Dot product
    float dot(const Vec2& other) const {
        return x * other.x + y * other.y;
    }

    // Distance to another vector
    float distance(const Vec2& other) const {
        return (*this - other).length();
    }

    // Distance squared (faster)
    float distanceSquared(const Vec2& other) const {
        return (*this - other).lengthSquared();
    }
};

#endif // VEC2_H
