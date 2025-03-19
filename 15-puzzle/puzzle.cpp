#include <iostream>

class Puzzle
{
private:
    int board[4][4] = { };
    int flatBoard[16] = { };
public:
    Puzzle(int board[4][4]) {
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 4; j++) {
                this->board[i][j] = board[i][j];
            }
        }
    }
    ~Puzzle(){
        delete this;
    }

    void printBoard() {
        std::cout << "_______________" << std::endl;
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 4; j++) {
            std::cout << " " << this->board[i][j] << " ";
            }
            std::cout << std::endl;
        }
        std::cout << "_______________" << std::endl;
    }


    void shuffleBoard(int times) {
        for (int i = 0; i < times; i++) {
            int rand1_row = rand() % 4;
            int rand1_col = rand() % 4;
            int rand2_row = rand() % 4;
            int rand2_col = rand() % 4;

            int temp = this->board[rand1_row][rand1_col];
            this->board[rand1_row][rand1_col] = this->board[rand2_row][rand2_col];
            this->board[rand2_row][rand2_col] = temp;
        }
    }

    bool isSolved() {
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 4; j++) {
                if (this->board[i][j] != i * 4 + j + 1) {
                    return false;
                }
            }
        }
        return true;
    }

    void flattenMatrix(int board[4][4]) {

        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 4; j++) {
                this->flatBoard[i * 4 + j] = board[i][j];
            }
        }
    }

    int mergeSort(int flatBoard[16], int low, int high) {

    

    }

    void solveBoard(){


    }

};


int main (int argc, char *argv[]) {
    int board[4][4] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0};
    Puzzle puzzle = Puzzle(board);
    std::cout << "Original Board: " << std::endl;
    puzzle.printBoard();
    puzzle.shuffleBoard(10);
    std::cout << "Shuffled Board: " << std::endl;
    puzzle.printBoard();

    return 0;
}
