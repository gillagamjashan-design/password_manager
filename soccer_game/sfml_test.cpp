// Simple SFML Installation Test
// This creates a window with a green circle to verify SFML works
//
// To compile and run this test:
//   g++ -std=c++17 sfml_test.cpp -o sfml_test -lsfml-graphics -lsfml-window -lsfml-system
//   ./sfml_test
//
// If you see a black window with a green circle, SFML is working correctly!

#include <SFML/Graphics.hpp>
#include <iostream>

int main() {
    std::cout << "=== SFML Installation Test ===" << std::endl;
    std::cout << "Creating window..." << std::endl;

    // Create a 400x300 window
    sf::RenderWindow window(sf::VideoMode(400, 300), "SFML Test - Close Me!");

    // Create a green circle
    sf::CircleShape circle(50.0f);
    circle.setFillColor(sf::Color::Green);
    circle.setPosition(175.0f, 100.0f);

    std::cout << "SUCCESS! Window created." << std::endl;
    std::cout << "If you see a green circle, SFML is working!" << std::endl;
    std::cout << "Close the window to exit this test." << std::endl;

    // Main loop
    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed)
                window.close();
        }

        window.clear(sf::Color::Black);
        window.draw(circle);
        window.display();
    }

    std::cout << "Test completed successfully! SFML is properly installed." << std::endl;
    std::cout << "You can now build and run the soccer game with: make run" << std::endl;

    return 0;
}
