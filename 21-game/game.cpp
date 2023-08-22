#include <cstdlib>
#include <ctime>
#include <iostream>
#include <random>
#include <string>
#include <vector>

class Game {
public:
  std::string player;
  std::vector<unsigned short> numbers = {1, 2, 3};

  void menu() {

    std::cout << "1. Play" << std::endl;
    std::cout << "2. Rules of the game" << std::endl;
    std::cout << "2. Exit" << std::endl;
    std::cout << "Choice: " << std::endl;
  }

  void rules() {
    std::cout << "Rules of the game" << std::endl;
    std::cout
        << "21 is a two player game, the game is played by choosing a number\n"
        << "(1, 2, or 3) to be added to the running total.\n"
        << "The game is won by the player whose chosen number\n"
        << "causes the running total to reach exactly 21.\n"
        << "The running total starts at zero. One player will be the\n"
        << "computer.\n"
        << "Players alternate supplying a number to be added to the running\n"
        << "total.\n"
        << std::endl;
  }
  unsigned short getNumber() {
    unsigned short random_index = std::rand() % numbers.size();
    unsigned short number = numbers[random_index];
    return number;
  }

  void play() {
    std::srand(static_cast<unsigned int>(std::time(nullptr)));

    unsigned short total_player = {0};
    unsigned short total_computer = {0};
    std::string playtime = {};

    std::cout << "Put your nickname: " << std::endl;
    std::cin >> player;

    std::cout << "Welcome " << player << "!" << std::endl;
    std::cout << "Let's play 21!\n"
              << "You gonna play against the computer." << std::endl;

    while (total_player < 21 && total_computer < 21) {
      std::cout << "Type 'y' to get a number or 'n' to pass your play to the "
                << "computer: " << std::endl;

      std::cin >> playtime;

      if (playtime == "y") {
        total_player += getNumber();
      } else {
        total_computer += getNumber();
      }

      std::cout << "Your total: " << total_player << std::endl;
      std::cout << "Computer total: " << total_computer << std::endl;
    }

    if (total_player > 21) {
      std::cout << "You lose!" << std::endl;
    } else if (total_computer > 21) {
      std::cout << "You win!" << std::endl;
    } else if (total_player == 21) {
      std::cout << "You win!" << std::endl;
    } else if (total_computer == 21) {
      std::cout << "You lose!" << std::endl;
    }
  }
};

int main(int argc, char *argv[]) {
  unsigned short choice = {0};

  Game game;

  game.menu();

  std::cin >> choice;

  switch (choice) {
  case 1:
    game.play();
    break;
  case 2:
    game.rules();
    break;
  case 3:
    break;
  default:
    break;
  }

  return 0;
}
