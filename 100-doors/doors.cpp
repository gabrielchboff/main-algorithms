#include <iostream>
#include <vector>

int main() {

  short steps = 1;
  std::vector<bool> doors(100, false);

  while (steps <= 100) {
    for (short i = 1; i <= 100; i += steps) {

      doors[i] = !doors[i];
    }
    steps += 1;
  }

  std::cout << "Last doors passed" << std::endl;
  for (bool door : doors) {
    if (door == 0){
      std::cout << "Door: " << "close" << std::endl;
    } else {
      std::cout << "Door: " << "open" << std::endl;

    }
   
  }
}
